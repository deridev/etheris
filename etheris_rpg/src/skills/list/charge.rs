use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Charge {
    charged: bool
}

#[async_trait::async_trait]
impl Skill for Charge {
    fn kind(&self) -> SkillKind {
        SkillKind::Charge
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "charge",
            name: "Investida",
            description: "Concentra e carrega uma poderosa investida para lançar com uma grande potência.",
            explanation: "Utilizar ether para fortalecer alguns músculos e estabilizar o equilíbrio do corpo é a preparação perfeita para uma investida de alta potência.",
            complexity: SkillComplexity::Simple,
            use_cost: SkillCost { ether: if self.charged { 5 } else { 10 } },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);
        display.sub_header.push_str(&format!("\n**Carregado**: {}", if self.charged { "Sim" } else { "Não" }));
        display
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        if self.charged {
            Probability::new(95)
        } else {
            Probability::new(50)
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if !self.charged {
            api.fighter_mut().add_balance(10);
            self.charged = true;
            api.emit_message(format!("**{}** carregou uma investida!", api.fighter().name));
            return Ok(());
        }

        self.charged = false;

        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let damage = api.rng().gen_range(15..=30);

        let multiplier = fighter.strength_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                kind: DamageKind::Physical,
                amount: damage,
                balance_effectiveness: 20,
                accuracy: 90,
                ..Default::default()
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** investiu e bateu com todo a força em **{}**, causando **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** deu uma poderosa investida em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        Ok(())
    }
}
