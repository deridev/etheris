use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Intimidation;

#[async_trait::async_trait]
impl Skill for Intimidation {
    fn kind(&self) -> SkillKind {
        SkillKind::Intimidation
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "intimidation",
            name: "Intimidação",
            description: "Emite uma aura ameaçadora, que faz todos da batalha focarem em você. Defende automaticamente após o uso.",
            explanation: "Habilidade simples, apenas emite ether descontroladamente para atrair atenção e se protege com ether depois.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: 8 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let ally_team = api.fighter().team;
        let fighter_index = api.fighter().index;

        for alive_index in api.battle().alive_fighters.clone() {
            let fighter = api.battle_mut().get_fighter_mut(alive_index);
            if fighter.team == ally_team {
                continue;
            }

            fighter.target = fighter_index;
            fighter.balance = fighter.balance.saturating_sub(5);
        }

        api.fighter_mut().defense += 2;

        format!("**{}** intimidou todos os inimigos e defendeu!", api.fighter().name);

        Ok(())
    }
}
