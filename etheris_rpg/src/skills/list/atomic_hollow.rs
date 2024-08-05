use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AtomicHollow;

fn calculate_cost(fighter: &Fighter) -> i32 {
    (fighter.ether.max / 2).max(40)
}

#[async_trait::async_trait]
impl Skill for AtomicHollow {
    fn kind(&self) -> SkillKind {
        SkillKind::AtomicHollow
    }

    fn data(&self, fighter: &Fighter) -> SkillData {
        let cost = calculate_cost(fighter);
        SkillData {
            identifier: "atomic_hollow",
            name: "Vazio Atômico",
            description: "Custa metade do ether máximo, e lança uma bola de ether condensado que aplica diversos efeitos e causa dano no alvo.",
            explanation: "Materializar uma bola de ether condensado com diversos efeitos requer massiva energia e controle do ether, além de ser perigoso de manusear, então requer um cuidado extra.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: cost },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        const EFFECT_KINDS: &[EffectKind] = &[EffectKind::Burning, EffectKind::Ice, EffectKind::Bleeding, EffectKind::Shocked];

        let base_damage = api.rng().gen_range(10..=15);
        let damage = api.rng().gen_range(3..=10);

        let multiplier = api.fighter().intelligence_multiplier();
        let damage = base_damage + ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::SpecialPhysical,
                amount: damage,
                balance_effectiveness: 25,
                accuracy: 99,
                effect: None
            },
        ).await;    

        if !damage.missed {
            for kind in EFFECT_KINDS {
                let amount = api.rng().gen_range(30..=60);
                api.apply_effect(target.index, Effect::new(*kind, amount, fighter.index)).await;
            }

            api.emit_message(format!("**{}** lançou uma esfera de vazio atômico em **{}** que causou **{damage}** e aplicou diversos efeitos no corpo do alvo.", fighter.name, target.name));
        } else {
            api.emit_message(format!("**{}** lançou uma esfera de vazio atômico em **{}**, mas o vazio se descontrolou e errou.", fighter.name, target.name));
        }


        api.add_overload(api.fighter_index, 3.5).await;

        Ok(())
    }
}
