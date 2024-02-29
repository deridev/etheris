use etheris_data::personality::Personality;

use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct WoundHealing;

#[async_trait::async_trait]
impl Skill for WoundHealing {
    fn kind(&self) -> SkillKind {
        SkillKind::WoundHealing
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "wound_healing",
            name: "Cicatrização",
            description: "Cura uma quantia pequena de vida de um aliado cicatrizando ferimentos",
            explanation: "Ether curativo é extremamente complexo, por isso mesmo curas simples como cicatrizações são complicadas de serem feitas.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 20 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_>) -> Probability {
        let fighter = api.fighter();
        let higher_chance = fighter.has_personality(Personality::Calm) || fighter.has_personality(Personality::Intelligence);

        if fighter.health().value < (fighter.health().max / 2) {
            Probability::new(if higher_chance { 80 } else { 55 })
        } else {
            Probability::new(45)
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter_index = api.fighter_index;
        let ally = api_input::select_ally(&mut api).await?;
        let Some(ally) = ally else {
            api.report_error("erro tentando selecionar o aliado.");
            return Ok(());
        };
        
        let cure = api.rng().gen_range(8.0..=10.0);
        let cure = 1 + (cure * (api.fighter().intelligence_multiplier() * 0.8)) as i32;

        {
            let ally = api.battle_mut().get_fighter_mut(ally.index);
            ally.heal(fighter_index, cure);
            ally.balance = ally.balance.saturating_add(20).clamp(0, 100);
            ally.defense = 1;
        }

        api.emit_message(format!("**{}** cicatrizou os ferimentos de **{}** e curou **{} vida**!", api.fighter().name, ally.name, cure));

        Ok(())
    }
}
