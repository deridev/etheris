use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct BloodTheft;

#[async_trait::async_trait]
impl Skill for BloodTheft {
    fn kind(&self) -> SkillKind {
        SkillKind::BloodTheft
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "blood_theft",
            name: "Roubo Sanguíneo",
            description: "Encosta sua mão no inimigo e rouba sua força vital para restaurar vida. Se o alvo estiver sangrando, você se cura mais.",
            explanation: "Habilidades de cura que surgem a partir do nada são extremamente complexas, por isso essa habilidade utiliza do sangue para obter energia vital. Quanto mais sangue, mais seu ether consegue roubar energia vital para si mesmo.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: 25 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let mut damage = api.rng().gen_range(3..=5);
        if let Some(bleeding) = target.get_effect(EffectKind::Bleeding) {
            damage += bleeding.amount / 8;
        }

        let multiplier = fighter.intelligence_multiplier() * 0.8;
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Special,
                amount: damage,
                balance_effectiveness: 0,
                accuracy: 100,
                ..Default::default()
            },
        ).await;


        api.emit_message(format!("**{}** tocou em **{}** e roubou **{damage}** para sua própria vitalidade!", fighter.name, target.name));

        api.fighter_mut().heal(fighter.index, damage.amount);
        

        Ok(())
    }
}
