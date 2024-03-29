use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct FinalCrucifix;

#[async_trait::async_trait]
impl Skill for FinalCrucifix {
    fn kind(&self) -> SkillKind {
        SkillKind::FinalCrucifix
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "final_crucifix",
            name: "Crucifixo Final",
            description: "Um ataque desesperado e final: sacrifica parte da sua vida para liberar uma explosão GIGANTE em forma de crucifixo. Dano em área. Depois de usar, seu ether não irá mais regenerar.",
            explanation: "Apesar de não ser muito complicado, pois é apenas uma explosão gigante em forma de crucifixo, utilizar ether em MASSIVAS quantidades não é para iniciantes. Essa habilidade ainda sacrifica sua regeneração natural de ether, que pode ser fatal se você não descansar logo após invocar o Crucifixo Final.",
            complexity: SkillComplexity::VeryHard,
            use_cost: SkillCost { ether: 100 },
        }
    }

    fn ai_chance_to_pick(&self, api: BattleApi<'_, '_>) -> Probability {
        if api.battle().turn_counter > 25 && api.fighter().health().value < api.fighter().health().max / 3 {
            Probability::new(30)
        } else {
            Probability::NEVER
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_, '_>) -> SkillResult<()> {
        let fighter_team = api.fighter().team;
        let multiplier = api.fighter().mixed_multiplier(0.1, 1.0);

        api.fighter_mut().flags.insert(FighterFlags::CANNOT_REGEN_ETHER);

        let self_damage = (api.rng().gen_range(20..=30) as f32 * multiplier) as i32;
        let ally_damage = (api.rng().gen_range(10..=20) as f32 * multiplier) as i32;
        let enemy_damage = (api.rng().gen_range(35..=45) as f32 * multiplier) as i32;

        let self_damage = api.apply_damage(api.fighter_index, DamageSpecifier {
            culprit: api.fighter_index,
            kind: DamageKind::Special,
            amount: self_damage,
            accuracy: 100,
            balance_effectiveness: 40,
            effect: None,
        }).await;

        let fighter_died = api.fighter().killed_by.is_some();
        if fighter_died {
            api.emit_message(format!("**{}** sacrificou toda sua vitalidade para invocar um Crucifixo Final, mas não sobreviveu ao próprio poder.", api.fighter().name));
        } else {
            api.emit_message(format!("**{}** sacrificou sua vitalidade e recebeu **{self_damage}** para invocar um magnífico Crucifixo Final!", api.fighter().name));
        }

        for index in api.battle().alive_fighters.clone() {
            if index == api.fighter_index {
                continue;
            }

            let fighter = api.battle().get_fighter(index).clone();
            let dmg = if fighter.team == fighter_team {
                ally_damage
            } else {
                enemy_damage
            };

            let dmg = api.apply_damage(index, DamageSpecifier {
                culprit: api.fighter_index,
                kind: DamageKind::Special,
                amount: dmg,
                accuracy: 100,
                balance_effectiveness: 30,
                effect: None,
            }).await;

            api.emit_random_message(&[
                format!("Uma poderosa explosão em forma de crucifixo acertou **{}** e causou **{dmg}**!", fighter.name),
                format!("**{}** recebeu uma explosão de energia vital e e recebeu **{dmg}**!", fighter.name),
                format!("**{}** foi explodido por um crucifixo final e recebeu **{dmg}**!", fighter.name),
            ]);
        }

        let overload = api.rng().gen_range(40.0..=60.0);
        api.add_overload(api.fighter_index, overload).await;

        Ok(())
    }
}
