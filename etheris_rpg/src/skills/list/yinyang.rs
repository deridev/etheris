use std::fmt::Display;

use etheris_data::personality::Personality;

use super::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YinYangState {
    #[default]
    Neutral,
    Yin,
    Yang
}

impl Display for YinYangState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            YinYangState::Neutral => "Neutro",
            YinYangState::Yin => "Yin",
            YinYangState::Yang => "Yang",
        };

        f.write_str(str)
    }
}

#[derive(Debug, Clone, Default)]
pub struct YinYang {
    state: YinYangState
}

#[async_trait::async_trait]
impl Skill for YinYang {
    fn kind(&self) -> SkillKind {
        SkillKind::YinYang
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "yinyang",
            name: "Yin-Yang",
            description: "Controla o estado da alma e alterna entre o modo Yin (ataque) ou o modo Yang (defesa).",
            explanation: "Alma é a base do ether, ether é a base da alma. Quando seu controle sobre o ether do seu corpo é preciso ao ponto de poder alterar sua alma, você pode alternar entre o estado de agressividade e proteção primordiais dos seres vivos. Requer extrema concentração e paz interior para alterar o estado da sua alma através do ether.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 20 },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);
        display.sub_header.push_str(&format!("\n**Estado**: {}", self.state));
        display
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        let fighter = api.fighter();
        if fighter.overload > 100.0 && self.state != YinYangState::Neutral {
            return Probability::new(95);
        }

        if fighter.overload > 50.0 {
            return Probability::new(20);
        }

        if self.state == YinYangState::Neutral {
            return Probability::new(60);
        }


        if fighter.has_personality(Personality::Aggressiveness) && self.state != YinYangState::Yin {
            return Probability::new(80);
        } else if fighter.has_personality(Personality::Calm) || fighter.has_personality(Personality::Intelligence) && self.state != YinYangState::Yang {
            return Probability::new(70);
        }

        Probability::new(10)
    }

    async fn passive_fighter_tick(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if self.state != YinYangState::Neutral {
            api.add_overload(api.fighter_index, 2.5).await;
        }
        Ok(())
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let next_state = match self.state {
            YinYangState::Neutral => YinYangState::Yin,
            YinYangState::Yin => YinYangState::Yang,
            YinYangState::Yang => YinYangState::Neutral,
        };

        api.emit_message(format!("**{}** concentrou e mudou o estado yin yang da sua alma para **{}**!", api.fighter().name, next_state));
        self.state = next_state;

        let fighter = api.fighter_mut();
        match next_state {
            YinYangState::Neutral => {
                fighter.modifiers.remove_all_with_tag("yan_defense");
            },
            YinYangState::Yin => {
                fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(2.0), None).with_tag("yin_dmg_boost"));
            },
            YinYangState::Yang => {
                fighter.modifiers.remove_all_with_tag("yin_dmg_boost");
                fighter.modifiers.add(Modifier::new(ModKind::DefenseMultiplier(0.5), None).with_tag("yan_defense"));
            }
        }

        let overload = api.rng().gen_range(3.0..=6.0);
        api.add_overload(api.fighter_index, overload).await;

        Ok(())
    }
}
