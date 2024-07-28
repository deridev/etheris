use std::cmp::Ordering;

use etheris_common::Probability;
use etheris_data::personality::Personality;
use rand::{seq::SliceRandom, Rng};

use crate::*;

pub async fn should_risk_life(api: BattleApi<'_>) -> bool {
    let fighter = api.fighter();
    let target = api.target();
    let pl_diff = fighter.pl - target.pl;

    let base_prob = fighter
        .personalities
        .iter()
        .map(|p| p.prob_of_risking_life().value() as i32)
        .sum::<i32>()
        / fighter.personalities.len() as i32;

    let mut prob = u8::try_from(base_prob).unwrap_or(50);

    // Consider health status
    let health_ratio = fighter.health().value as f64 / fighter.health().max as f64;
    prob = prob.saturating_add((health_ratio * 20.0) as u8);

    // Consider target's status
    if target.flags.contains(FighterFlags::RISKING_LIFE) {
        prob = prob.saturating_add(if pl_diff > 0 { 30 } else { 15 });
    } else {
        prob = prob.saturating_sub(5);
    }

    // Consider team situation
    let team_members = api
        .battle()
        .teams()
        .get(&fighter.team)
        .cloned()
        .unwrap_or_default();
    let team_health_avg = team_members
        .iter()
        .map(|&index| {
            let member = api.battle().get_fighter(index);
            member.health().value as f64 / member.health().max as f64
        })
        .sum::<f64>()
        / team_members.len() as f64;

    prob = prob.saturating_add((team_health_avg * 10.0) as u8);

    // Consider remaining ether
    let ether_ratio = fighter.ether.value as f64 / fighter.ether.max as f64;
    prob = prob.saturating_add((ether_ratio * 15.0) as u8);

    Probability::new(prob).generate_random_bool()
}

pub async fn allow_fighter_to_enter_his_team(api: BattleApi<'_>, _index: FighterIndex) -> bool {
    let Some(team) = api.battle().teams().get(&api.fighter().team).cloned() else {
        return false;
    };

    let is_there_a_human_on_the_team = team
        .iter()
        .any(|index| api.battle().get_fighter(*index).user.is_some());

    // Allow the human to decide if the fighter can enter his team
    // (is this case, the unnecessary if-bool improves legibility and reduces cognitive overload of this function)
    #[allow(clippy::needless_bool)]
    if is_there_a_human_on_the_team {
        true
    } else {
        false
    }
}

pub async fn select_target(api: &mut BattleApi<'_>) {
    let fighter = api.fighter().clone();

    // Prioritize targets based on threat level and team strategy
    let mut potential_targets: Vec<_> = api
        .battle()
        .alive_fighters
        .iter()
        .map(|&index| api.battle().get_fighter(index))
        .filter(|f| f.team != fighter.team && !f.is_defeated)
        .collect();

    potential_targets.sort_by(|a, b| {
        let a_threat = calculate_threat_level(api, a);
        let b_threat = calculate_threat_level(api, b);
        b_threat.partial_cmp(&a_threat).unwrap_or(Ordering::Equal)
    });

    if let Some(new_target) = potential_targets.first() {
        api.fighter_mut().target = new_target.index;
    } else {
        api.battle_mut().reallocate_fighter_target(fighter.index);
    }
}

pub fn calculate_threat_level(api: &BattleApi<'_>, target: &Fighter) -> f64 {
    let pl_ratio = target.pl as f64 / api.fighter().pl as f64;
    let health_ratio = target.health().value as f64 / target.health().max as f64;
    let skill_threat: f64 = target
        .skills
        .iter()
        .map(|s| s.base_kind.knowledge_cost() as f64)
        .sum::<f64>()
        / target.skills.len() as f64;

    pl_ratio * 0.4 + (1.0 - health_ratio) * 0.3 + skill_threat * 0.3
}

pub async fn select_a_input(mut api: BattleApi<'_>) -> BattleInput {
    let fighter = api.fighter().clone();
    let target = api.target().clone();

    if api.can_finish_target() {
        return select_finisher(&fighter, api.rng());
    }

    if fighter.composure == Composure::OnGround {
        return handle_ground_situation(&fighter, &target);
    }

    let skills = evaluate_skills(&fighter, &mut api).await;

    if !skills.is_empty() && should_use_skill(&fighter, &skills, api.rng()) {
        return BattleInput::UseSkill(skills[0].clone());
    }

    if should_defend(&fighter, &target) {
        return BattleInput::Defend;
    }

    BattleInput::Attack
}

fn select_finisher(fighter: &Fighter, rng: &mut impl Rng) -> BattleInput {
    let finisher = fighter
        .finishers
        .choose_weighted(rng, |f| {
            let base_weight = if f.is_fatal() { 1 } else { 5 };
            let personality_modifier = if fighter.has_personality(Personality::Aggressiveness) {
                2
            } else {
                1
            };
            base_weight * personality_modifier
        })
        .expect("Finishers should not be empty");

    BattleInput::Finish(*finisher)
}

fn handle_ground_situation(fighter: &Fighter, target: &Fighter) -> BattleInput {
    let mut upkick_prob = if target.health().value < (target.health().max / 4) {
        Probability::new(30)
    } else {
        Probability::new(10)
    };

    if fighter.has_personality(Personality::Aggressiveness) {
        upkick_prob.add(40);
    } else if fighter.has_personality(Personality::Courage) {
        upkick_prob.add(15);
    }

    if upkick_prob.generate_random_bool() {
        BattleInput::Upkick
    } else {
        BattleInput::GetUp
    }
}

async fn evaluate_skills(fighter: &Fighter, api: &mut BattleApi<'_>) -> Vec<FighterSkill> {
    let mut evaluated_skills = vec![];

    for skill in fighter.skills.iter() {
        let dyn_skill = skill.dynamic_skill.lock().await;
        if dyn_skill.can_use(BattleApi::new(api.controller)) {
            let chance = dyn_skill.ai_chance_to_pick(BattleApi::new(api.controller));
            if chance.generate_random_bool() {
                evaluated_skills.push((skill.clone(), chance.value()));
            }
        }
    }

    evaluated_skills.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    evaluated_skills
        .into_iter()
        .map(|(skill, _)| skill)
        .collect()
}

fn should_use_skill(fighter: &Fighter, skills: &[FighterSkill], rng: &mut impl Rng) -> bool {
    if skills.is_empty() {
        return false;
    }

    let skill_use_chance = if fighter.has_personality(Personality::Intelligence) {
        0.8
    } else if fighter.has_personality(Personality::Aggressiveness) {
        0.6
    } else {
        0.5
    };

    rng.gen_bool(skill_use_chance)
}

fn should_defend(fighter: &Fighter, target: &Fighter) -> bool {
    if fighter.has_personality(Personality::Insanity) {
        return false;
    }

    let health_ratio = fighter.health().value as f64 / fighter.health().max as f64;
    let ether_ratio = fighter.ether.value as f64 / fighter.ether.max as f64;
    let target_health_ratio = target.health().value as f64 / target.health().max as f64;

    let base_defend_chance = if fighter.has_personality(Personality::Calm)
        || fighter.has_personality(Personality::Intelligence)
    {
        0.3
    } else {
        0.1
    };

    let health_factor = (1.0 - health_ratio) * 0.3;
    let ether_factor = if ether_ratio <= 0.5 { 0.2 } else { 0.05 };
    let target_factor = target_health_ratio * 0.15;

    let total_chance = base_defend_chance + health_factor + ether_factor - target_factor;

    rand::thread_rng().gen_bool(total_chance)
}
