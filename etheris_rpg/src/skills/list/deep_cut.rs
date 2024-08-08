use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DeepCut {
    pub charged: bool
}

#[async_trait::async_trait]
impl Skill for DeepCut {
    fn kind(&self) -> SkillKind {
        SkillKind::DeepCut
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        SkillData {
            identifier: "deep_cut",
            name: "Corte Profundo",
            description: "Emite um corte profundo no alvo materializado com ether. Requer concentração para materializar, portanto precisa de um bom equilíbrio.", 
            explanation: "Habilidade de alto nível composta por duas fases: primeira, imaginar o corte no objeto-alvo, e em seguida, usar ether para romper o objeto em forma de corte. Era uma habilidade muito usada por organizações mercenárias na era de ouro da espada.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: if self.charged { 45 } else { 15 } },
        }
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        if self.charged {
            display.sub_header.push_str("\n**Carregado**");
        }

        display
    }

    
    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        if self.charged {
            Probability::new(95)
        } else {
            Probability::new(51)
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        let required_balance = if self.charged { 50 } else { 82 };
        api.fighter().balance > required_balance && self.default_can_use(api)
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if !self.charged {
            api.fighter_mut().add_balance(10);
            self.charged = true;
            api.emit_message(format!("**{}** carregou um corte profundo!", api.fighter().name));
            return Ok(());
        }

        self.charged = false;

        let fighter = api.fighter().clone();
        let target = api.target().clone();

        let base_damage = api.rng().gen_range(30..=50);
        let damage = base_damage + api.rng().gen_range(15..=20);

        let multiplier = fighter.intelligence_multiplier();
        let damage = ((damage as f32) * multiplier) as i32;

        let damage = api.apply_damage(
            target.index,
            DamageSpecifier {
                culprit: fighter.index,
                kind: DamageKind::Cut,
                amount: damage,
                balance_effectiveness: 15,
                accuracy: 100,
                effect: Some(Effect::new(EffectKind::Bleeding, 50, fighter.index))
            },
        ).await;

        api.emit_random_message(&[
            format!(
                "**{}** disparou um corte profundo em **{}** que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** rasgou **{}** com um corte profundo que causou **{damage}**!",
                fighter.name, target.name
            ),
            format!(
                "**{}** se concentrou e materializou um corte profundo em **{}**, causando **{damage}**!",
                fighter.name, target.name
            ),
        ]);

        if api.target().vitality.value <= 0 {
            api.emit_random_message(&[
                format!("**{}** foi cortado ao meio.", api.target().name),
                format!("**{}** teve o tronco cortado.", api.target().name),
                format!("**{}** foi dilacerado.", api.target().name),
                format!("**{}** foi separado em duas partes.", api.target().name),
            ]);
        }

        Ok(())
    }
}
