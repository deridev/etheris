use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct WaterBlessing;

#[async_trait::async_trait]
impl Skill for WaterBlessing {
    fn kind(&self) -> SkillKind {
        SkillKind::WaterBlessing
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "water_blessing",
            name: "Bênção da Água",
            description: "Despeja água abençoada com ether em um aliado, fortalecendo seu corpo por alguns turnos.",
            explanation: "Requer materialização de uma boa quantia de água, e então enviar um fluxo de ether com propriedades fortalecedoras pelo líquido. ",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 18 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        
        let ally = api_input::select_ally(&mut api).await?;

        let Some(ally) = ally else {
            api.report_error("erro tentando selecionar o aliado.");
            return Ok(());
        };

        api.apply_effect(ally.index, Effect::new(EffectKind::Wet, 100, fighter.index)).await;
        api.apply_effect(ally.index, Effect::new(EffectKind::LowProtection, 5, fighter.index)).await;

        if ally.index == fighter.index {
            api.emit_message(format!("**{}** despejou água abençoada em si mesmo e aumentou sua proteção por alguns turnos!", fighter.name));
        } else {
            api.emit_message(format!("**{}** despejou água abençoada em **{}** e aumentou sua proteção por alguns turnos!", fighter.name, ally.name));
        }

        Ok(())
    }
}
