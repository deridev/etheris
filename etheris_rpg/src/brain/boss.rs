#![allow(dead_code)]
use std::collections::HashMap;

use brain::Brain;
use etheris_common::Probability;
use etheris_data::{personality::Personality, BrainKind};
use etheris_database::character_model::BattleAction;
use rand::{seq::IteratorRandom, Rng};

use crate::*;

#[derive(Debug, Clone)]
pub struct BossBrain {
    phase: BossPhase,
    action_weights: HashMap<BossPhase, HashMap<BattleInputKind, f64>>,
    memory: Vec<BattleState>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum BossPhase {
    Opening,
    Aggressive,
    Defensive,
    Desperate,
}

#[derive(Debug, Clone)]
struct BattleState {
    fighter_health_ratio: f64,
    target_health_ratio: f64,
    fighter_ether_ratio: f64,
    turn_count: usize,
    last_target_input: BattleInput,
    composure: Composure,
}

impl BossBrain {
    pub fn new() -> Self {
        let mut action_weights = HashMap::new();

        for phase in &[
            BossPhase::Opening,
            BossPhase::Aggressive,
            BossPhase::Defensive,
            BossPhase::Desperate,
        ] {
            let mut phase_weights = HashMap::new();
            for input_kind in BattleInputKind::LIST.iter() {
                phase_weights.insert(*input_kind, 1.0);
            }
            action_weights.insert(phase.clone(), phase_weights);
        }

        BossBrain {
            phase: BossPhase::Opening,
            action_weights,
            memory: Vec::new(),
        }
    }

    fn determine_phase(&mut self, health_ratio: f64) {
        self.phase = match health_ratio {
            x if x > 0.75 => BossPhase::Opening,
            x if x > 0.4 => BossPhase::Aggressive,
            x if x > 0.2 => BossPhase::Defensive,
            _ => BossPhase::Desperate,
        };
    }

    async fn adjust_weights(
        &mut self,
        battle_state: &BattleState,
        fighter: &Fighter,
        mut api: BattleApi<'_>,
    ) {
        let phase_weights = self.action_weights.get_mut(&self.phase).unwrap();

        *phase_weights.get_mut(&BattleInputKind::Finish).unwrap() = 10.0;
        *phase_weights.get_mut(&BattleInputKind::Defend).unwrap() = 0.45;
        *phase_weights.get_mut(&BattleInputKind::UseItem).unwrap() = 0.1;
        *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() = 2.5;

        // Reset weights for actions that can't be used in current composure
        for (input_kind, weight) in phase_weights.iter_mut() {
            if !input_kind.can_use(BattleApi::new(api.controller)) {
                *weight = 0.0;
            }
        }

        if (self.phase == BossPhase::Aggressive
            || self.phase == BossPhase::Opening
            || api.rng().gen_bool(0.01))
            && fighter.actions.contains(&BattleAction::ControlPower)
            && fighter.potential > fighter.power
        {
            *phase_weights.get_mut(&BattleInputKind::Actions).unwrap() = 0.8;
        } else {
            *phase_weights.get_mut(&BattleInputKind::Actions).unwrap() = 0.0;
        }

        // Check if there's a high probability of a skill being used
        for skill in fighter.skills.iter().cloned() {
            let prob = skill
                .dynamic_skill
                .lock()
                .await
                .ai_chance_to_pick(BattleApi::new(api.controller));

            if prob.value() >= 99 {
                *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() = 10.0;
            } else if prob.value() > 75 {
                *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() += 0.7;
            }
        }

        // Adjust weights based on battle state and composure
        match battle_state.composure {
            Composure::Standing => {
                if battle_state.fighter_health_ratio < 0.3 {
                    *phase_weights.get_mut(&BattleInputKind::Defend).unwrap() += 0.1;
                }
            }
            Composure::OnGround => {
                *phase_weights.get_mut(&BattleInputKind::GetUp).unwrap() += 1.0;
                if battle_state.target_health_ratio < 0.5 {
                    *phase_weights.get_mut(&BattleInputKind::Upkick).unwrap() += 0.5;
                }
            }
            Composure::OnAir(_) => {
                *phase_weights.get_mut(&BattleInputKind::Attack).unwrap() += 0.5;
                *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() += 0.5;
            }
        }

        // Adjust weights based on fighter's personalities
        for personality in &fighter.personalities {
            match personality {
                Personality::Aggressiveness => {
                    *phase_weights.get_mut(&BattleInputKind::Attack).unwrap() += 0.3;
                    *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() += 0.2;
                }
                Personality::Intelligence => {
                    *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() += 0.4;
                    *phase_weights.get_mut(&BattleInputKind::UseItem).unwrap() += 0.2;
                }
                Personality::Calm => {
                    *phase_weights.get_mut(&BattleInputKind::Defend).unwrap() += 0.2;
                }
                // Add more personality-based adjustments as needed
                _ => {}
            }
        }

        // Analyze recent target actions
        if self.memory.len() >= 3 {
            let recent_actions: Vec<BattleInput> = self
                .memory
                .iter()
                .rev()
                .take(3)
                .map(|state| &state.last_target_input)
                .cloned()
                .collect();

            if recent_actions
                .iter()
                .all(|action| matches!(action, &BattleInput::Attack))
            {
                *phase_weights.get_mut(&BattleInputKind::Defend).unwrap() += 0.2;
            } else if recent_actions
                .iter()
                .all(|action| matches!(action, &BattleInput::Defend))
            {
                *phase_weights.get_mut(&BattleInputKind::UseSkill).unwrap() += 0.4;
            }
        }
    }

    fn select_action(&self, api: BattleApi<'_>) -> BattleInputKind {
        let phase_weights = self.action_weights.get(&self.phase).unwrap();
        let mut rng = rand::thread_rng();

        let total_weight: f64 = phase_weights.values().sum();
        let mut random_value = rng.gen::<f64>() * total_weight;

        for (action, weight) in phase_weights {
            if action.can_use(BattleApi::new(api.controller)) {
                random_value -= weight;
                if random_value <= 0.0 {
                    return *action;
                }
            }
        }

        // Fallback to a safe action
        if api.fighter().composure == Composure::OnGround {
            BattleInputKind::GetUp
        } else {
            BattleInputKind::Defend
        }
    }
}

#[async_trait::async_trait]
impl Brain for BossBrain {
    fn kind(&self) -> BrainKind {
        BrainKind::Boss
    }

    async fn should_risk_life(&mut self, api: BattleApi<'_>) -> bool {
        if api.fighter().boss.is_some() {
            return true;
        }

        if api.fighter().personalities.contains(&Personality::Insanity) {
            return true;
        }

        let fighter = api.fighter();
        let base_prob = fighter
            .personalities
            .iter()
            .map(|p| p.prob_of_risking_life().value() as i32)
            .sum::<i32>()
            / fighter.personalities.len() as i32;

        let mut prob = Probability::new(u8::try_from(base_prob).unwrap_or(50));

        if self.phase == BossPhase::Desperate {
            prob.add(30);
        }

        prob.generate_random_bool()
    }

    async fn select_input(&mut self, mut api: BattleApi<'_>) -> BattleInput {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let health_ratio = fighter.health().value as f64 / fighter.health().max as f64;
        self.determine_phase(health_ratio);

        let battle_state = BattleState {
            fighter_health_ratio: health_ratio,
            target_health_ratio: target.health().value as f64 / target.health().max as f64,
            fighter_ether_ratio: fighter.ether.value as f64 / fighter.ether.max as f64,
            turn_count: api.battle().turn_counter,
            last_target_input: BattleInput::Attack,
            composure: fighter.composure,
        };

        self.adjust_weights(&battle_state, &fighter, BattleApi::new(api.controller))
            .await;

        let selected_action = self.select_action(BattleApi::new(api.controller));

        match selected_action {
            BattleInputKind::UseSkill => {
                if let Some(skill) = self.select_skill(api).await {
                    BattleInput::UseSkill(skill)
                } else {
                    BattleInput::Attack // Fallback if no skill is available
                }
            }
            BattleInputKind::Finish => {
                if api.can_finish_target() {
                    self.select_finisher(&fighter, api.rng())
                } else {
                    BattleInput::Attack // Fallback if can't finish
                }
            }
            BattleInputKind::UseItem => {
                if let Some(item) = fighter.inventory.iter().choose(api.rng()) {
                    BattleInput::UseItem(item.item)
                } else {
                    BattleInput::Attack // Fallback if no item is available
                }
            }
            BattleInputKind::Actions => {
                use rand::prelude::SliceRandom;
                let action = fighter.actions.choose_weighted(api.rng(), |a| match a {
                    BattleAction::ControlPower => 50,
                    BattleAction::GiveUp => 0,
                });

                if let Ok(action) = action {
                    BattleInput::UseAction(action.clone())
                } else {
                    BattleInput::Attack // Fallback if no action is available
                }
            }
            BattleInputKind::Attack => BattleInput::Attack,
            BattleInputKind::Defend => BattleInput::Defend,
            BattleInputKind::GetUp => BattleInput::GetUp,
            BattleInputKind::Upkick => BattleInput::Upkick,
            BattleInputKind::ChangeTarget => {
                // Simple logic to change target, can be improved
                let new_target = api
                    .battle()
                    .alive_fighters
                    .iter()
                    .map(|&index| api.battle().get_fighter(index).clone())
                    .filter(|f| f.team != fighter.team)
                    .find(|f| f.index != fighter.index && f.index != fighter.target)
                    .map(|f| f.index)
                    .unwrap_or(fighter.target);
                BattleInput::ChangeTarget(new_target)
            }
            BattleInputKind::ChangeTeam => BattleInput::Attack, // fallback
        }
    }

    async fn allow_fighter_to_enter_his_team(
        &mut self,
        _api: BattleApi<'_>,
        _fighter: FighterIndex,
    ) -> bool {
        // Boss doesn't allow others to join its team
        false
    }
}

impl BossBrain {
    async fn select_skill(&self, api: BattleApi<'_>) -> Option<FighterSkill> {
        let fighter = api.fighter();
        let mut evaluated_skills = vec![];

        for skill in fighter.skills.clone() {
            let dyn_skill = skill.dynamic_skill.lock().await;
            if dyn_skill.can_use(BattleApi::new(api.controller)) {
                let mut chance = dyn_skill.ai_chance_to_pick(BattleApi::new(api.controller));

                // Adjust chance based on boss phase
                if self.phase == BossPhase::Aggressive {
                    chance.add(5);
                }

                if chance.generate_random_bool() {
                    evaluated_skills.push((skill.clone(), chance.value()));
                }
            }
        }

        evaluated_skills.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        evaluated_skills.first().map(|(skill, _)| skill.clone())
    }

    fn select_finisher<RNG: Rng>(&self, fighter: &Fighter, rng: &mut RNG) -> BattleInput {
        use rand::prelude::SliceRandom;
        let finisher = fighter
            .finishers
            .choose_weighted(rng, |f| {
                let base_weight = if f.is_fatal() { 1 } else { 3 };
                let personality_modifier =
                    if fighter.personalities.contains(&Personality::Aggressiveness) {
                        2
                    } else {
                        1
                    };
                base_weight * personality_modifier
            })
            .expect("Finishers should not be empty");

        BattleInput::Finish(*finisher)
    }
}
