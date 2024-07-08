use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct InstinctiveReaction;

#[async_trait::async_trait]
impl Skill for InstinctiveReaction {
    fn kind(&self) -> SkillKind {
        SkillKind::InstinctiveReaction
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "instinctive_reaction",
            name: "Reação Instintiva",
            description: "Habilidade passiva. Quando alguém errar um ataque em você, tem 80% de chance de instintivamente dar um ataque de volta.",
            explanation: "Essa habilidade requer extrema concentração e controle do ether. Se baseia em ter ether fluindo no corpo esperando a oportunidade de atacar antes mesmo do cérebro mandar o ataque.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 10 },
        }
    }

    fn can_use(&self, _api: BattleApi<'_>) -> bool {
        false
    }

    async fn passive_on_damage_miss(&mut self, mut api: BattleApi<'_>, _damage: DamageSpecifier) -> SkillResult<()> {
        if api.fighter_index == api.target_index {
            return Ok(());
        }

        let fighter = api.fighter().clone();
        if fighter.ether.value < self.data(&fighter).use_cost.ether {
            return Ok(());
        }

        if Probability::new(20).generate_random_bool() {
            return Ok(());
        }

        api.fighter_mut().ether.value -= self.data(&fighter).use_cost.ether;

        let base_damage = api.rng().gen_range(4..=10);
        let damage = api.rng().gen_range(12..=16);
        let damage = base_damage + (damage as f32 * api.fighter().mixed_multiplier(0.9, 0.1)) as i32;

        let damage = api
            .apply_damage(
                api.target_index,
                DamageSpecifier {
                    culprit: fighter.index,
                    amount: damage,
                    kind: DamageKind::Physical,
                    balance_effectiveness: 15,
                    accuracy: 100,
                    effect: None,
                },
            )
            .await;

        api.emit_message(format!("**{}** instintivamente atacou e deu **{damage}** em **{}**!", api.fighter().name, api.target().name));
    
        Ok(())
    }

    async fn on_use(&mut self, mut _api: BattleApi<'_>) -> SkillResult<()> {
        Ok(())
    }
}
