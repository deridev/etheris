use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct EtherFlow {
    active: bool,
    turns_counter: u32,
}

#[async_trait::async_trait]
impl Skill for EtherFlow {
    fn kind(&self) -> SkillKind {
        SkillKind::EtherFlow
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "ether_flow",
            name: "Fluxo de Ether",
            description: "Aumenta temporariamente a regeneração de ether, mas reduz ligeiramente o dano causado.",
            explanation: "Ao alterar o fluxo de ether no corpo, é possível acelerar sua regeneração às custas de potência ofensiva.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 20 },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);
        display.sub_header.push_str(&format!("\n**Ativo**: {}", if self.active { "Sim" } else { "Não" }));
        if self.active {
            display.sub_header.push_str(&format!("\n**Turnos restantes**: {}", self.turns_counter));
        }
        display
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        !self.active && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if self.active {
            api.emit_message(format!("**{}** já está em Fluxo de Ether!", api.fighter().name));
            return Ok(());
        }

        self.active = true;
        self.turns_counter = 5;

        let fighter = api.fighter_mut();
        fighter.modifiers.add(Modifier::new(ModKind::EtherRegenMultiplier(1.65), Some(3)).with_tag("ether_flow_regen"));
        fighter.modifiers.add(Modifier::new(ModKind::DmgMultiplier(0.9), Some(3)).with_tag("ether_flow_damage"));

        let fighter_name = fighter.name.clone();
        api.emit_message(format!("**{}** ativou o Fluxo de Ether, aumentando sua regeneração de ether!", fighter_name)); 

        Ok(())
    }

    async fn passive_fighter_tick(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if self.active {
            self.turns_counter -= 1;
            if self.turns_counter <= 0 {
                self.active = false;
                self.turns_counter = 0;

                let fighter = api.fighter_mut();
                fighter.modifiers.remove_all_with_tag("ether_flow_regen");
                fighter.modifiers.remove_all_with_tag("ether_flow_damage");
                
                let fighter_name = fighter.name.clone();
                api.emit_message(format!("O Fluxo de Ether de **{}** se dissipou!", fighter_name));
            }
        }
        Ok(())
    }
}