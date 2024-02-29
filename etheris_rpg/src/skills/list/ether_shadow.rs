use etheris_common::Attribute;

use self::brain::BrainKind;

use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct EtherShadow {
    used: bool
}

#[async_trait::async_trait]
impl Skill for EtherShadow {
    fn kind(&self) -> SkillKind {
        SkillKind::EtherShadow
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "ether_shadow",
            name: "Sombra de Ether",
            description: "Invoca uma sombra sua feita de ether para ajudar na batlha. Um uso por batalha.",
            explanation: "Habilidade de materialização simples, mas requer entendimento profundo do seu próprio ether para invocar uma sombra sua.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 60 },
        }
    }

    
    fn can_use(&self, api: BattleApi<'_>) -> bool {
        !self.used && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        self.used = true;

        let fighter = api.fighter().clone();

        let mut skills = vec![];
        for skill in fighter.skills {
            let kind = skill.base_kind;
            if kind == self.kind() {
                continue;
            }

            skills.push(kind);
        }

        let resistance = (fighter.resistance.max as f32 * 0.4) as i32;
        let vitality = (fighter.vitality.max as f32 * 0.4) as i32;
        let ether = (fighter.ether.max as f32 * 0.5) as i32;

        api.battle_mut().join_fighter(FighterData { 
            team: fighter.team, 
            name: format!("Sombra de {}", fighter.name), 
            user: None, 
            brain: Some(BrainKind::Simple),
            inventory: vec![],
            personalities: fighter.personalities.clone(), 
            skills, 
            strength_level: 1 + (fighter.strength_level as f32 * 0.3) as u32, 
            intelligence_level: 1 + (fighter.intelligence_level as f32 * 0.3) as u32, 
            weapon: fighter.weapon.map(|w| w.kind), 
            resistance: Attribute::from(resistance), 
            vitality: Attribute::from(vitality),
            ether: Attribute::from(ether), 
            drop: Default::default()
        });

        api.emit_message(format!("**{}** invocou uma sombra de si mesmo para ajudar na batalha!", fighter.name));

        let overload = api.rng().gen_range(3.0..=5.0);
        api.add_overload(api.fighter_index, overload).await;

        Ok(())
    }
}
