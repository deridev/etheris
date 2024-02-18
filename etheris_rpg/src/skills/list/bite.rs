use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Bite;

#[async_trait::async_trait]
impl Skill for Bite {
    fn kind(&self) -> SkillKind {
        SkillKind::Bite
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "bite",
            name: "Mordida",
            description: "Uma simples mordida com força no corpo do inimigo, mas que suga um pouco de ether.",
            explanation: "Dependendo da profundidade da mordida, é possível surrupiar um pouco do fluxo de ether do alvo, mas ether de terceiros é perigoso então apenas pequenas quantidades podem ser absorvidas.",
            complexity: SkillComplexity::VerySimple,
            use_cost: SkillCost { ether: 5 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(2..=5);

        let multiplier = fighter.mixed_multiplier(0.7, 0.4);
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::PhysicalCut,
                amount: damage,
                balance_effectiveness: 5,
                accuracy: 80,
                ..Default::default()
            },
        ).await;

        if damage.missed {
            api.emit_message(format!("**{}** tentou morder **{}** mas errou!", fighter.name, target.name))
        } else {
            let ether_stealed = (target.ether.value as f32 * 0.2) as i32;
            api.emit_message(if ether_stealed > 0 {
                format!("**{}** mordeu **{}** e deu **{damage}**, além de roubar **`{ether_stealed} ether`** na mordida!", fighter.name, target.name)
            } else {
                format!("**{}** mordeu **{}** e deu **{damage}**!", fighter.name, target.name)
            });

            api.fighter_mut().ether.add(ether_stealed);
            api.target_mut().ether.remove(ether_stealed);
        }

        Ok(())
    }
}
