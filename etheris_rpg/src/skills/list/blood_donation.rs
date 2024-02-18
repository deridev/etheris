use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct BloodDonation;

#[async_trait::async_trait]
impl Skill for BloodDonation {
    fn kind(&self) -> SkillKind {
        SkillKind::BloodDonation
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "blood_donation",
            name: "Doação Sanguínea",
            description: "Doa seu sangue para um aliado, para poder curar um pouco seus ferimentos.",
            explanation: "Por mais simples que pareça, requer extremo controle do ether para manter a vitalidade no seu sangue doado e impedir que seu ether danifique o aliado. De resto basta beber vitalidade líquida.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 25 },
        }
    }

    fn can_use(&self, api: BattleApi<'_, '_>) -> bool {
        let allies = api.get_fighter_allies(api.fighter_index);
        allies.len() > 1 && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();

        let ally = api_input::select_ally(&mut api).await?;
        let Some(ally) = ally else {
            api.report_error("erro tentando selecionar o aliado.");
            return Ok(());
        };

        let hp = ((fighter.health().value as f32) * 0.1) as i32;

        api.fighter_mut().take_damage(
            fighter.index, 
            DamageSpecifier { kind: DamageKind::Special, amount: hp, balance_effectiveness: 0, accuracy: 100, effect: None, culprit: fighter.index }
        );
        api.battle_mut().get_fighter_mut(ally.index).heal(fighter.index, hp);

        api.emit_message(format!("**{}** bebeu sangue de **{}**, e roubou **{} vida**!", ally.name, fighter.name, hp));

        Ok(())
    }
}
