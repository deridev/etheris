use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Pyrotransmutation {
    turn_count: u8,
    active: bool
}

#[async_trait::async_trait]
impl Skill for Pyrotransmutation {
    fn kind(&self) -> SkillKind {
        SkillKind::Pyrotransmutation
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "pyrotransmutation",
            name: "Pirotransmutação",
            description: "Transforma seu corpo em fogo, se tornando imune a fogo e aumentando seu dano em 50% por 4 rodadas.",
            explanation: "Uma habilidade poderosa e complexa. Moldando a propriedade física do seu corpo temporariamente, o corpo evolui por alguns minutos. Extremamente perigoso, mas poderoso.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 30 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        !self.active && self.default_can_use(api)
    }

    
    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        if self.active {
            display.sub_header.push_str("\n**Ativo**");
        }
        
        display
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        self.active = true;
        self.turn_count = 4;

        let fighter = api.fighter_mut();
        fighter.body_immunities.add_resistance(ImmunityKind::Fire, 1.0);
        fighter.body_immunities.add_weakness(ImmunityKind::Water, 1.0);
        fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(1.5), None).with_tag("pyrotransmutation_dmg"));

        api.emit_message(format!("**{}** usou a pirotransmutação!", api.fighter().name));

        Ok(())
    }

    async fn passive_on_cycle(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if !self.active {
            return Ok(());
        }

        if self.turn_count == 0 {
            self.active = false;
            
            let fighter = api.fighter_mut();
            fighter.body_immunities.remove_resistance(ImmunityKind::Fire, 1.0);
            fighter.body_immunities.remove_weakness(ImmunityKind::Water, 1.0);
            fighter.modifiers.remove_all_with_tag("pyrotransmutation_dmg");

            api.emit_message(format!("A pirotransmutação de **{}** terminou!", api.fighter().name));
        }

        self.turn_count -= 1;

        Ok(())
    }
}
