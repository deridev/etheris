use etheris_data::personality::Personality;

use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct ParalyzingBet;

#[async_trait::async_trait]
impl Skill for ParalyzingBet {
    fn kind(&self) -> SkillKind {
        SkillKind::ParalyzingBet
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "paralyzing_bet",
            name: "Aposta Paralisante",
            description: "55% de chance de paralisar o inimigo e 45% de chance de paralisar a si mesmo. A paralisia é de dois turnos.",
            explanation: "Baseada no conceito de metaconsciência do ether, essa habilidade expande o limite do que o ether do seu corpo pode fazer adicionando uma restrição com consequências ao uso dessa habilidade, assim permitindo o efeito de paralisar os músculos ser efetuado ao emitir uma forma de ether caótica e liderada pela aleatoriedade no corpo do afetado para prender os músculos por um tempo.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 30 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_, '_>) -> Probability {
        if api.fighter().has_personality(Personality::Insanity) {
            Probability::new(60)
        } else {
            Probability::new(25)
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let unlucky_index = if Probability::new(55).generate_random_bool() {
            target.index
        } else {
            fighter.index
        };

        let unlucky = api.battle().get_fighter(unlucky_index).clone();

        api.emit_message(format!("A sorte foi lançada e **{}** teve os músculos paralizados!", unlucky.name));
        api.apply_effect(unlucky.index, Effect::new(EffectKind::Paralyzed, 2, fighter.index)).await;

        api.add_overload(api.fighter_index, 2.0).await;

        Ok(())
    }
}
