use etheris_common::Probability;
use etheris_data::personality::Personality;
use rand::{seq::SliceRandom, Rng};

use crate::*;

pub async fn default_should_risk_life(api: BattleApi<'_>) -> bool {
    let fighter = api.fighter();

    let probs = fighter
        .personalities
        .iter()
        .map(|p| p.prob_of_risking_life());
    let prob = probs.map(|p| p.value() as i32).sum::<i32>() / fighter.personalities.len() as i32;

    let prob = u8::try_from(prob).unwrap_or(50);

    Probability::new(prob).generate_random_bool()
}

pub async fn allow_fighter_to_enter_his_team(
    mut _api: BattleApi<'_>,
    _index: FighterIndex,
) -> bool {
    false
}

pub async fn select_target(api: &mut BattleApi<'_>) {
    let fighter = api.fighter().clone();

    if let Some(state) = fighter.ai_state {
        let wants_to_change_target =
            Probability::new(if fighter.has_personality(Personality::Aggressiveness) {
                95
            } else {
                70
            })
            .generate_random_bool();

        if wants_to_change_target
            && state.focused_in != fighter.target
            && !api.battle().get_fighter(state.focused_in).is_defeated
            && api.battle().get_fighter(state.focused_in).team != fighter.team
        {
            api.fighter_mut().target = state.focused_in;
        }
    }

    if Probability::new(40).generate_random_bool() {
        api.battle_mut().reallocate_fighter_target(fighter.index);
    }

    if api.battle().get_fighter(fighter.target).team == api.fighter().team
        || api.battle().get_fighter(fighter.target).is_defeated
    {
        api.battle_mut().reallocate_fighter_target(fighter.index);
    }
}

pub async fn select_a_input(mut api: BattleApi<'_>) -> BattleInput {
    let fighter = api.fighter().clone();

    if api.can_finish_target() {
        let finisher = fighter
            .finishers
            .choose_weighted(api.rng(), |f| if f.is_fatal() { 1 } else { 5 })
            .expect("Finishers should not be empty");
        return BattleInput::Finish(*finisher);
    }

    if fighter.composure == Composure::OnGround {
        let target = api.target();
        let mut upkick_prob = if target.health().value < (target.health().max / 4) {
            Probability::new(20)
        } else {
            Probability::new(5)
        };

        if fighter.has_personality(Personality::Aggressiveness) {
            upkick_prob.add(40);
        } else if fighter.has_personality(Personality::Courage) {
            upkick_prob.add(10);
        }

        if upkick_prob.generate_random_bool() {
            return BattleInput::Upkick;
        }

        return BattleInput::GetUp;
    }

    let mut skills = vec![];
    let mut high_skill_priority = false;
    let mut low_skill_priority = true;

    for skill in fighter.skills.iter() {
        let dyn_skill = skill.dynamic_skill.lock().await;
        if dyn_skill.can_use(BattleApi::new(api.controller)) {
            let prob = dyn_skill.ai_chance_to_pick(BattleApi::new(api.controller));

            if prob.value() > 80 {
                high_skill_priority = true;
            }

            if prob.value() >= 50 {
                low_skill_priority = false;
            }

            if prob.generate_random_bool() {
                skills.push(skill.clone());
            }
        }
    }

    if high_skill_priority
        || api
            .rng()
            .gen_bool(if low_skill_priority { 0.05 } else { 0.4 })
    {
        if let Some(skill) = skills.choose(&mut api.rng()) {
            return BattleInput::UseSkill((*skill).clone());
        }
    }

    if !fighter.has_personality(Personality::Insanity)
        && fighter.health().value != fighter.health().max
        && api
            .rng()
            .gen_bool(if fighter.ether.value < (fighter.ether.max / 2) {
                0.5
            } else {
                0.2
            })
    {
        return BattleInput::Defend;
    }

    BattleInput::Attack
}
