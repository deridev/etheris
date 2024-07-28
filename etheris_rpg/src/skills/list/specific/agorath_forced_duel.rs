use super::super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AgorathForcedDuel {
    duel_target: Option<FighterIndex>,
    last_punishment_turn: usize,
}

#[async_trait::async_trait]
impl Skill for AgorathForcedDuel {
    fn kind(&self) -> SkillKind {
        SkillKind::AgorathForcedDuel
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "agorath_forced_duel",
            name: "Duelo Forçado",
            description: "Força a batalha a ser um duelo de verdade.",
            explanation: "Habilidade única de um poderoso duelista.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost {
                ether: if self.duel_target.is_none() { 0 } else { 25 },
            },
        }
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        if self.duel_target.is_none() {
            Probability::ALWAYS
        } else {
            Probability::NEVER
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let target = api
            .battle()
            .alive_fighters
            .iter()
            .map(|f| api.battle().get_fighter(*f))
            .cloned()
            .collect::<Vec<_>>();

        let Some(target) = target
            .iter()
            .find(|f| f.user.is_some() && f.index != api.fighter().index)
        else {
            api.emit_message(format!(
                "**{}** não conseguiu encontrar um alvo para um duelo de verdade!",
                api.fighter().name
            ));
            return Ok(());
        };

        self.duel_target = Some(target.index);
        api.emit_message(format!(
            "**{}** forçou a batalha a ser um duelo de verdade contra **{}**!",
            api.fighter().name,
            target.name
        ));

        Ok(())
    }

    async fn passive_on_cycle(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let Some(allowed_index) = self.duel_target else {
            return Ok(());
        };

        if api.battle().turn_counter == self.last_punishment_turn {
            return Ok(());
        }

        let fighters = api
            .battle()
            .alive_fighters
            .iter()
            .map(|f| api.battle().get_fighter(*f))
            .cloned()
            .collect::<Vec<_>>();

        for fighter in fighters {
            if fighter.index == api.fighter().index || fighter.index == allowed_index {
                continue;
            }

            let dmg = (fighter.health().max as f32 * api.rng().gen_range(0.2..=0.4)) as i32;

            let dmg = api
                .apply_damage(
                    fighter.index,
                    DamageSpecifier {
                        culprit: api.fighter().index,
                        kind: DamageKind::Special,
                        accuracy: 255,
                        amount: dmg,
                        balance_effectiveness: 30,
                        effect: Some(Effect::new(EffectKind::Bleeding, 40, fighter.index)),
                    },
                )
                .await;

            api.emit_message(format!(
                "**{}** recebeu a punição por invadir o duelo e sofreu **{dmg}**!",
                fighter.name
            ));
        }

        self.last_punishment_turn = api.battle().turn_counter;

        Ok(())
    }
}
