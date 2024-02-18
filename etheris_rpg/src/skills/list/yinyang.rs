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

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "yinyang",
            name: "Yin-Yang",
            description: "Controla o estado da alma e alterna entre o modo Yin (ataque) ou o modo Yang (defesa).",
            explanation: "Alma é a base do ether, ether é a base da alma. Quando seu controle sobre o ether do seu corpo é preciso ao ponto de poder alterar sua alma, você pode alternar entre o estado de agressividade e proteção primordiais dos seres vivos. Requer extrema concentração e paz interior para alterar o estado da sua alma através do ether.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 20 },
        }
    }

    fn display(&self) -> SkillDisplay {
        let mut display = self.default_display();
        display.sub_header.push_str(&format!("\n**Estado**: {}", self.state));
        display
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_, '_>) -> Probability {
        if self.state == YinYangState::Neutral {
            return Probability::new(60);
        }

        let fighter = api.fighter();

        if fighter.has_personality(Personality::Aggressiveness) && self.state != YinYangState::Yin {
            return Probability::new(80);
        } else if fighter.has_personality(Personality::Calm) || fighter.has_personality(Personality::Intelligence) && self.state != YinYangState::Yang {
            return Probability::new(70);
        }

        Probability::new(10)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
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
                fighter.defense = 0;
            },
            YinYangState::Yin => {
                fighter.strength_level *= 2;
            },
            YinYangState::Yang => {
                fighter.strength_level /= 2;
                api.fighter_mut().defense = 1_000_000; // arbitrary large number
            }
        }

        Ok(())
    }
}
