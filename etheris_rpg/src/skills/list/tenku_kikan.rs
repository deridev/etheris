use etheris_data::Soul;

use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct TenkuKikan {
    pub soul: Option<Soul>,
}

impl TenkuKikan {
    pub fn new(soul: Option<Soul>) -> Self {
        Self {
            soul
        }
    }
}

#[async_trait::async_trait]
impl Skill for TenkuKikan {
    fn kind(&self) -> SkillKind {
        SkillKind::TenkuKikan(self.soul.clone())
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "tenku_kikan",
            name: "Tenkū Kikan",
            description: "Armazena a alma do último que morreu pelas suas mãos e cria uma cópia para lutar ao seu lado. A alma só pode ser usada uma vez.",
            explanation: "Alma é ether: guardar ether de alguém que já morreu é extremamente complicado. O ether se desfaz na morte, então é necessário agir rápido para armazenar ether em uma área neutra do seu corpo, efetivamente armazenando a alma do seu inimigo junto. Como ether é vida, na morte o ether da alma enfraquece. Depois é só materializar um corpo e colocar o resquício da alma que você armazenou.",
            complexity: SkillComplexity::VeryHard,
            use_cost: SkillCost { ether: 80 },
        }
    }

    fn can_use(&self, api: BattleApi<'_, '_>) -> bool {
        self.default_can_use(api) && self.soul.is_some()
    }

    fn display(&self) -> SkillDisplay {
        let mut display = self.default_display();

        if let Some(soul) = &self.soul {
            display.sub_header.push_str(&format!("\n**Alma**: {}", soul.name));
        }

        display
    }

    async fn passive_on_kill(&mut self, mut api: BattleApi<'_, '_>, killed: FighterIndex) -> SkillResult<()> {
        let killed = api.battle().get_fighter(killed);

        let mut skills = Vec::with_capacity(killed.skills.len());
        for skill in killed.skills.iter() {
            let skill_kind = skill.dynamic_skill.lock().await.kind();
            skills.push(skill_kind);
        }

        let soul = Soul {
            name: killed.name.to_owned(),
            ether: killed.ether.max,
            resistance: killed.resistance.max,
            vitality: killed.vitality.max,
            strength: killed.strength_level,
            intelligence: killed.intelligence_level,
            personalities: killed.personalities.to_owned(),
            skills,
        };

        api.defer_message(format!("**{}** usou **{}** e armazenou a alma de **{}**!", api.fighter().name, self.data().name, soul.name));

        self.soul = Some(soul);

        Ok(())
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let Some(mut soul) = self.soul.clone() else {
            api.emit_message(format!("**{}** tentou usar Tenkū Kikan sem nenhuma alma armazenada e desperdiçou ether.", api.fighter().name));
            return Ok(());
        };

        let team = api.fighter().team;
        self.soul = None;

        api.emit_message(format!("**{}** sacrificou os resquícios da alma de **{}** para invocar uma réplica com força inferior!", api.fighter().name, soul.name));

        soul.ether = (soul.ether as f32 * 0.6) as i32;
        soul.resistance = (soul.resistance as f32 * 0.6) as i32;
        soul.vitality = (soul.vitality as f32 * 0.6) as i32;
        soul.intelligence = (soul.intelligence as f32 * 0.6) as u32;
        soul.strength = (soul.strength as f32 * 0.6) as u32;
        api.battle_mut().join_fighter(FighterData {
            team,
            name: format!("{} (Réplica)", soul.name),
            user: None,

            drop: Default::default(),
            personalities: soul.personalities.clone(),

            ether: soul.ether.into(),
            resistance: soul.resistance.into(),
            vitality: soul.vitality.into(),

            strength_level: soul.strength,
            intelligence_level: soul.intelligence,

            weapon: None,
            skills: soul.skills,
        });

        Ok(())
    }
}
