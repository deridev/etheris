use std::ops::Add;

use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct MirrorDamage {
    pub accumulated_damage: i32
}

#[async_trait::async_trait]
impl Skill for MirrorDamage {
    fn kind(&self) -> SkillKind {
        SkillKind::MirrorDamage
    }

    fn data(&self, _fighter: &Fighter) -> SkillData {
        let ether_cost = match self.accumulated_damage {
            0 => 0,
            1..=10 => 5,
            11..=50 => 10,
            51..=150 => 20,
            151..=200 => 35,
            201..=500 => 50, 
            _ => 80
        };

        SkillData {
            identifier: "mirror_damage",
            name: "Espelhar Dano",
            description: "Acumula 40% de todo o dano físico recebido e então retorna de uma vez só. O dano acumulado não passa de 1000.",
            explanation: "A energia ether é tão versátil que com treinamento suficiente é possível armazenar impacto físico de forma segura no fluxo de ether do corpo, e então basta liberar esse ether com impacto acumulado no alvo e o dano se transfere.",
            complexity: SkillComplexity::Normal,
            use_cost: SkillCost { ether: ether_cost },
        }
    }

    fn can_use(&self, api: BattleApi<'_>) -> bool {
        self.accumulated_damage > 0 && self.default_can_use(api)
    }

    fn display(&self, fighter: &Fighter) -> SkillDisplay {
        let mut display = self.default_display(fighter);

        display.sub_header.push_str(&format!("\n**Acumulado**: {} dano", self.accumulated_damage));

        display
    }

    fn ai_chance_to_pick(&self, _api: BattleApi<'_>) -> Probability {
        if self.accumulated_damage < 30 {
            Probability::NEVER
        } else if self.accumulated_damage < 500 {
            Probability::new(30)
        } else {
            Probability::new(60)
        }
    }

    async fn passive_fighter_tick(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        if self.accumulated_damage > 0 && api.fighter().ether.value <= 0 {
            api.emit_message(format!("**{}** ficou sem ether e perdeu o dano acumulado na habilidade **{}**", api.fighter().name, self.data(api.fighter()).name));
            self.accumulated_damage = 0;
        }

        Ok(())
    }

    async fn passive_on_damage(&mut self, mut api: BattleApi<'_>, damage: DamageSpecifier) -> SkillResult<()> {
        let damage = match damage.kind {
            DamageKind::Physical | DamageKind::PhysicalCut => (damage.amount as f32 * 0.4) as i32,
            DamageKind::SpecialPhysical => (damage.amount as f32 * 0.35) as i32,
            DamageKind::Cut => (damage.amount as f32 * 0.15) as i32,
            _ => 0
        };

        if api.fighter().ether.value < 1 || damage < 1 || api.fighter().is_defeated {
            return Ok(());
        }

        self.accumulated_damage = self.accumulated_damage.add(damage).min(1000);

        if self.accumulated_damage == 1000 {
            api.emit_message(format!("**{}** acumulou o dano máximo na habilidade **{}**", api.fighter().name, self.data(api.fighter()).name));
        } else {
            api.emit_message(format!("**{}** acumulou **{damage} dano** na habilidade **{}**", api.fighter().name, self.data(api.fighter()).name));
        }
        
        Ok(())
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let damage = DamageSpecifier {
            kind: DamageKind::Physical,
            amount: self.accumulated_damage,
            balance_effectiveness: if self.accumulated_damage < 50 { 5 } else { 10 },
            accuracy: 95,
            ..Default::default()
        };

        let damage = api.apply_damage(api.target_index, damage).await;
        self.accumulated_damage = 0;

        api.emit_message(
            format!("**{}** refletiu **{damage}** em **{}**", api.fighter().name, api.target().name)
        );

        Ok(())
    }
}
