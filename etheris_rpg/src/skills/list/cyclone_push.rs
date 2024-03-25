use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct CyclonePush;

#[async_trait::async_trait]
impl Skill for CyclonePush {
    fn kind(&self) -> SkillKind {
        SkillKind::CyclonePush
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "cyclone_push",
            name: "Empurrão Ciclone",
            description: "Lança um forte vento para empurrar o seu inimigo.",
            explanation: "Espalha seu ether pelo vento ao seu redor e então basta controlar o ether espalhado para mover o ar com velocidade e força até seu inimigo.",
            complexity: SkillComplexity::VerySimple,
            use_cost: SkillCost { ether: 6 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(3..=7);

        let multiplier = fighter.intelligence_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Wind,
                amount: damage,
                balance_effectiveness: 35,
                accuracy: 90,
                ..Default::default()
            },
        ).await;

        api.emit_message(
            format!(
                "**{}** lançou um ciclone de vento para empurrar **{}**, causando **{damage}** e tirando equilíbrio!",
                fighter.name, target.name
            ),

        );

        Ok(())
    }
}
