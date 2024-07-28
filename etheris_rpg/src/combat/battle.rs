use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::{Add, Sub},
};

use etheris_data::world::regions::WorldRegion;
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    SeedableRng,
};

use crate::{
    common::DamageSpecifier, EffectKind, Fighter, FighterData, FighterFlags, FighterIndex,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BattleError {
    NeedsTwoFighters,
}

impl Display for BattleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NeedsTwoFighters => f.write_str("Battle must have at least two fighters"),
        }
    }
}

impl std::error::Error for BattleError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TurnHistory {
    pub round: usize,
    pub fighter: FighterIndex,
    pub target: FighterIndex,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BattleState {
    Running,
    Ended {
        winner_team: u8,
        winners: Vec<FighterIndex>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BattleSettings {
    pub is_risking_life_allowed: bool,
    pub casual: bool,
    pub has_consequences: bool,
    pub max_intruders: u8,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            is_risking_life_allowed: true,
            has_consequences: true,
            casual: false,
            max_intruders: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TurnEndQueues {
    pub damages: Vec<(DamageSpecifier, FighterIndex)>,
    pub damage_misses: Vec<(DamageSpecifier, FighterIndex)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Battle {
    pub rng: StdRng,
    pub fighters: Vec<Fighter>,
    pub alive_fighters: Vec<FighterIndex>,
    pub defeated_fighters: Vec<FighterIndex>,
    pub fighters_queue: VecDeque<FighterIndex>,
    pub current_fighter: FighterIndex,
    pub region: WorldRegion,

    pub history: Vec<TurnHistory>,
    pub turn_counter: usize,
    pub cycle_counter: usize,
    pub intruder_count: usize,

    pub state: BattleState,
    pub settings: BattleSettings,

    pub turn_end_queues: TurnEndQueues,
    pub deferred_turn_messages: Vec<String>,
}

impl Battle {
    pub fn new(
        region: WorldRegion,
        settings: BattleSettings,
        fighters: Vec<FighterData>,
    ) -> Result<Battle, BattleError> {
        if fighters.len() < 2 {
            return Err(BattleError::NeedsTwoFighters);
        }

        let number_of_fighters = fighters.len();

        let alive_fighters = (0..fighters.len()).map(FighterIndex).collect::<Vec<_>>();
        let fighters = fighters
            .into_iter()
            .enumerate()
            .map(|(index, data)| {
                Fighter::new(
                    data.team,
                    FighterIndex(index),
                    FighterIndex((index + 1) % number_of_fighters),
                    data,
                )
            })
            .collect::<Vec<_>>();

        let mut battle = Battle {
            settings,
            rng: StdRng::from_entropy(),
            fighters,
            fighters_queue: alive_fighters.clone().into(),
            alive_fighters,
            defeated_fighters: vec![],
            current_fighter: FighterIndex(usize::MAX / 2), // Dummy value
            region,
            history: vec![],
            turn_counter: 0,
            cycle_counter: 0,
            intruder_count: 0,
            state: BattleState::Running,
            turn_end_queues: TurnEndQueues::default(),
            deferred_turn_messages: Vec::new(),
        };

        // Fix targets
        battle.reallocate_all_targets();

        if battle.settings.casual {
            for fighter in &mut battle.fighters {
                fighter.regenerate_all();
            }
        }

        // Set the current fighter
        battle.next_fighter();

        Ok(battle)
    }

    pub fn full_teams(&self) -> HashMap<u8, Vec<Fighter>> {
        let mut map: HashMap<u8, Vec<Fighter>> = HashMap::with_capacity(self.fighters.len());
        for fighter in self.fighters.iter() {
            let fighters = map.entry(fighter.team).or_default();
            fighters.push(fighter.clone());
        }

        map
    }

    pub fn teams(&self) -> HashMap<u8, Vec<FighterIndex>> {
        let mut map: HashMap<u8, Vec<FighterIndex>> = HashMap::with_capacity(self.fighters.len());
        for fighter in self.alive_fighters.iter() {
            let fighter = self.get_fighter(*fighter);
            let fighters = map.entry(fighter.team).or_default();
            fighters.push(fighter.index);
        }

        map
    }

    pub fn join_fighter(&mut self, fighter_data: FighterData) {
        let index = FighterIndex(self.fighters.len());
        let mut fighter = Fighter::new(fighter_data.team, index, FighterIndex(0), fighter_data);
        if self.settings.casual {
            fighter.regenerate_all();
        }

        self.fighters.push(fighter);
        self.alive_fighters.push(index);
        self.reallocate_fighter_target(index);
        self.fighters_queue.push_back(index);
    }

    pub fn join_intruder(&mut self, fighter_data: FighterData) {
        self.join_fighter(fighter_data);
        self.intruder_count += 1;
    }

    pub fn reallocate_fighter_target(&mut self, fighter_index: FighterIndex) {
        let fighter = self.get_fighter(fighter_index).clone();
        let teams = self.teams();

        let Some((.., enemies)) = teams
            .iter()
            .filter(|(team, ..)| **team != fighter.team)
            .choose(&mut self.rng)
        else {
            return;
        };

        let Some(enemy) = enemies.choose(&mut self.rng) else {
            return;
        };

        let fighter = self.get_fighter_mut(fighter_index);
        fighter.target = *enemy;
    }

    pub fn reallocate_all_targets(&mut self) {
        for fighter in self.alive_fighters.clone() {
            self.reallocate_fighter_target(fighter);
        }
    }

    pub fn next_turn(&mut self, this_turn_history: &mut TurnHistory) {
        self.turn_counter += 1;

        // Deferred turn messages
        for message in self.deferred_turn_messages.clone() {
            this_turn_history.messages.push(message);
        }

        self.deferred_turn_messages.clear();

        let mut fighters_defeated_in_this_turn = vec![];
        for alive_fighter in self.alive_fighters.iter().copied() {
            let fighter = &self.fighters[alive_fighter.0];
            if fighter.is_defeated {
                fighters_defeated_in_this_turn.push(alive_fighter);
            }
        }

        for fighter in fighters_defeated_in_this_turn {
            // Check if this fighter is the last alive: If it is, don't make it lose. This is to prevent the game from ending in a tie.
            {
                let alive_fighters = self.alive_fighters.to_vec();
                if alive_fighters.contains(&fighter) && alive_fighters.len() == 1 {
                    continue;
                }
            }

            self.defeated_fighters.push(fighter);
            self.alive_fighters.retain(|f| f.0 != fighter.0);
            self.fighters_queue.retain(|f| f.0 != fighter.0);

            let fighter = self.get_fighter(fighter).clone();
            this_turn_history.messages.push(format!(
                "**{}** {}.",
                fighter.name,
                if fighter.vitality.value <= 0 {
                    "morreu"
                } else {
                    "desmaiou"
                }
            ));
        }

        let teams = self.teams();
        if teams.keys().count() < 2 {
            let winner_team = teams
                .keys()
                .nth(0)
                .copied()
                .unwrap_or(self.fighters[0].team);
            if teams.get(&winner_team).is_none() {
                this_turn_history
                    .messages
                    .push(format!("ERRO: Time {winner_team} não encontrado."));
                return;
            }

            let winners = teams[&winner_team].to_owned();

            self.state = BattleState::Ended {
                winner_team,
                winners: winners.clone(),
            };

            if winners.len() == 1 {
                let winner = self.get_fighter(winners[0]);
                this_turn_history
                    .messages
                    .push(format!("> :medal: **{}** venceu!", winner.name));
            } else {
                let team_winners = winners
                    .iter()
                    .map(|w| self.get_fighter(*w).clone())
                    .collect::<Vec<_>>();
                this_turn_history.messages.push(format!(
                    "> :medal: O time **[{winner_team}]** venceu! Os vencedores são: `{}`",
                    team_winners
                        .iter()
                        .map(|w| w.name.to_owned())
                        .collect::<Vec<_>>()
                        .join("`, `")
                ));
            }
        }

        // Tick every fighter
        for alive_fighter in self.alive_fighters.clone() {
            let fighter = &mut self.fighters[alive_fighter.0];
            let fighter_team = fighter.team;
            let fighter_is_ai = fighter.ai_state.is_some();
            let target = fighter.target;

            fighter.recalculate_pl();

            let ether_rec = if fighter.flags.contains(FighterFlags::CANNOT_REGEN_ETHER)
                || fighter
                    .flags
                    .contains(FighterFlags::CANNOT_REGEN_ETHER_OVERLOAD)
                || fighter.has_effect(EffectKind::Exhausted)
            {
                0
            } else {
                (fighter.ether.max as f32 * 0.05) as i32
            };

            let ether_rec = (ether_rec as f32 * fighter.modifiers.overall_ether_regen_multiplier())
                .max(0.0) as i32;

            fighter.overload = if ether_rec == 0 {
                fighter.overload
            } else {
                fighter.overload.sub(0.3).clamp(0.0, 1000.0)
            };

            fighter.balance = fighter.balance.add(3).min(100);
            fighter.defense = fighter.defense.saturating_sub(1);
            fighter.ether.add(ether_rec);

            if self.get_fighter(target).is_defeated
                || (self.get_fighter(target).team == fighter_team && fighter_is_ai)
            {
                self.reallocate_fighter_target(alive_fighter);
            }
        }

        self.turn_end_queues = Default::default();
        self.history.push(this_turn_history.clone());
    }

    pub fn next_fighter(&mut self) {
        if self.fighters_queue.is_empty() {
            let hasnt_gave_up =
                |f: &&FighterIndex| !self.get_fighter(**f).flags.contains(FighterFlags::GAVE_UP);
            self.fighters_queue = self
                .alive_fighters
                .iter()
                .filter(hasnt_gave_up)
                .cloned()
                .collect::<Vec<_>>()
                .into();
        }

        if self.fighters_queue.is_empty() {
            return;
        }

        let new_fighter = self.fighters_queue.pop_front().unwrap();
        self.current_fighter = new_fighter;
    }

    pub fn get_fighter(&self, index: FighterIndex) -> &Fighter {
        &self.fighters[index.0]
    }

    pub fn get_fighter_mut(&mut self, index: FighterIndex) -> &mut Fighter {
        &mut self.fighters[index.0]
    }

    pub fn get_current_fighter(&self) -> &Fighter {
        self.get_fighter(self.current_fighter)
    }

    pub fn get_current_fighter_mut(&mut self) -> &mut Fighter {
        self.get_fighter_mut(self.current_fighter)
    }

    pub fn get_target_fighter(&self) -> &Fighter {
        self.get_fighter(self.get_current_fighter().target)
    }

    pub fn get_target_fighter_mut(&mut self) -> &mut Fighter {
        self.get_fighter_mut(self.get_current_fighter().target)
    }
}
