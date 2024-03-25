use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct CursedBlood;

#[async_trait::async_trait]
impl Skill for CursedBlood {
    fn kind(&self) -> SkillKind {
        SkillKind::CursedBlood
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "cursed_blood",
            name: "Sangue Amaldiçoado",
            description: "Usando seu sangue ou do seu alvo, aplica uma maldição no seu alvo que o faz tomar o dano que causa aos outros. Requer que você ou o alvo estejam sangrando.",
            explanation: "Não se sabe quem criou a Maldição do Karma: *Soridu'Karmi*, mas ela é uma poderosa maldição que faz com que todo o dano que você causa para alguém volte em parte para você. A habilidade do Sangue Amaldiçoado usa ether para aplicar a maldição no alvo, sendo assim uma habilidade complexa e poderosa.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 35 },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
       (api.fighter().has_effect(EffectKind::Bleeding) || api.target().has_effect(EffectKind::Bleeding)) && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let amount = if target.has_effect(EffectKind::Curse) {
            api.rng().gen_range(25..=30)
        } else {
            api.rng().gen_range(40..=55)
        };

        api.apply_effect(target.index, Effect::new(EffectKind::Curse, amount, fighter.index)).await;

        api.emit_message(format!("**{}** aplicou uma maldição em **{}** através do sangue!", api.fighter().name, target.name));
        Ok(())
    }
}
