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

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "tenku_kikan",
            name: "Tenkū Kikan",
            description: "Armazena a alma do último que morreu pelas suas mãos e cria uma cópia para lutar ao seu lado. A alma só pode ser usada uma vez.",
            explanation: "Alma é ether: guardar ether de alguém que já morreu é extremamente complicado. O ether se desfaz na morte, então é necessário agir rápido para armazenar ether em uma área neutra do seu corpo, efetivamente armazenando a alma do seu inimigo junto. Como ether é vida, na morte o ether da alma enfraquece. Depois é só materializar um corpo e colocar o resquício da alma que você armazenou.",
            complexity: SkillComplexity::VeryHard,
            use_cost: SkillCost { ether: 80 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        self.default_can_use(api) && self.soul.is_some()
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        Probability::new(70)
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        if let Some(soul) = &self.soul {
            display.sub_header.push_str(&format!("\n**Alma**: {}", soul.name));
        }

        display
    }

    async fn passive_on_kill(&mut self, mut api: BattleApi<'_>, killed: FighterIndex) -> SkillResult<()> {
        let killed = api.battle().get_fighter(killed);
        if killed.vitality.value > 0 || api.fighter().vitality.value <= 0 {
            return Ok(());
        }
        
        if self.soul.is_some() {
            api.defer_message(format!("**{}** não conseguiu armazenar a alma de **{}** no {} pois já há uma alma armazenada!", api.fighter().name, killed.name, self.data(api.fighter()).name));
            return Ok(());
        }

        let mut skills = Vec::with_capacity(killed.skills.len());
        for skill in killed.skills.iter() {
            let skill_kind = skill.dynamic_skill.lock().await.kind();
            skills.push(skill_kind);
        }

        let mut pacts = Vec::with_capacity(killed.pacts.len());
        for pact in killed.pacts.iter() {
            let pact_kind = pact.base_kind.clone();
            pacts.push(pact_kind);
        }

        let soul = Soul {
            name: killed.name.to_owned(),
            brain: killed.brain.as_ref().map(|b| b.kind),
            ether: killed.ether.max,
            resistance: killed.resistance.max,
            vitality: killed.vitality.max,
            strength: killed.strength_level,
            intelligence: killed.intelligence_level,
            personalities: killed.personalities.to_owned(),
            skills,
            pacts,
        };

        api.defer_message(format!("**{}** usou **{}** e armazenou a alma de **{}**!", api.fighter().name, self.data(api.fighter()).name, soul.name));

        self.soul = Some(soul);

        Ok(())
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let Some(mut soul) = self.soul.clone() else {
            api.emit_message(format!("**{}** tentou usar Tenkū Kikan sem nenhuma alma armazenada e desperdiçou ether.", api.fighter().name));
            return Ok(());
        };

        let team = api.fighter().team;
        self.soul = None;

        api.emit_message(format!("**{}** sacrificou os resquícios da alma de **{}** para invocar uma réplica com força inferior!", api.fighter().name, soul.name));

        soul.ether = (soul.ether as f32 * 0.7) as i32;
        soul.resistance = (soul.resistance as f32 * 0.6) as i32;
        soul.vitality = (soul.vitality as f32 * 0.7) as i32;
        soul.intelligence = (soul.intelligence as f32 * 0.6) as u32;
        soul.strength = (soul.strength as f32 * 0.45) as u32;
        api.battle_mut().join_fighter(FighterData {
            team,
            name: format!("{} (Réplica)", soul.name),
            user: None,
            brain: Some(soul.brain.unwrap_or_default()),
            actions: vec![],
            boss: None,

            inventory: vec![],
            drop: Default::default(),
            potential: 0.5,
            personalities: soul.personalities.clone(),

            ether: soul.ether.into(),
            resistance: soul.resistance.into(),
            vitality: soul.vitality.into(),

            strength_level: soul.strength,
            intelligence_level: soul.intelligence,

            weapon: None,
            skills: soul.skills,
            pacts: soul.pacts,
            immunities: BodyImmunities::new()
        });

        api.add_overload(api.fighter_index, 40.0).await;

        Ok(())
    }
}
