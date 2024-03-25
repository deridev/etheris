use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct BloodSpear;

#[async_trait::async_trait]
impl Skill for BloodSpear {
    fn kind(&self) -> SkillKind {
        SkillKind::BloodSpear
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "blood_spear",
            name: "Lança de Sangue",
            description: "Materializa uma lança feita do seu sangue e atira no seu inimigo, transferindo parte do seu sangramento para ele.",
            explanation: "A materialização através do ether é extremamente complexa, no entanto, a Lança de Sangue utiliza o próprio sangue como matéria para criar o objeto, sendo assim não muito difícil.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 30 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        api.fighter().has_effect(EffectKind::Bleeding) && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let mut damage = api.rng().gen_range(15..=20);
        let bleeding = fighter.get_effect(EffectKind::Bleeding).unwrap();
        damage += bleeding.amount / 15;

        let multiplier = fighter.intelligence_multiplier() * 0.8;
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::PhysicalCut,
                amount: damage,
                balance_effectiveness: 15,
                accuracy: 90,
                ..Default::default()
            },
        ).await;

        if damage.missed {
            api.emit_message(format!("**{}** tentou atirar uma lança de sangue em **{}** mas errou!", fighter.name, target.name))
        } else {
            let bleeding_transferred = (bleeding.amount as f32 * 0.5) as i32;

            api.emit_message(format!("**{}** materializou uma lança de sangue e atirou em **{}**, causando **{damage}** e transferindo um pouco de sangramento!", fighter.name, target.name));

            api.fighter_mut().remove_effect(Effect::new(EffectKind::Bleeding, bleeding_transferred, fighter.index));
            api.target_mut().apply_effect(Effect::new(EffectKind::Bleeding, bleeding_transferred, target.index));
        }        

        Ok(())
    }
}
