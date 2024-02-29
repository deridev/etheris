use super::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Earthquake;

#[async_trait::async_trait]
impl Skill for Earthquake {
    fn kind(&self) -> SkillKind {
        SkillKind::Earthquake
    }

    fn data(&self) -> SkillData {
        SkillData {
            identifier: "earthquake",
            name: "Terremoto",
            description: "Emite ether pelos seus pés até o chão, tremendo toda a arena e dando dano em todos os inimigos. O dano em aliados é menor.",
            explanation: "Requer uma quantia alta de ether destrutivo sendo emitido pelo seu pé até o fundo do chão da arena. O controle preciso do ether é a chave aqui.",
            complexity: SkillComplexity::Hard,
            use_cost: SkillCost { ether: 30 },
        }
    }

    async fn on_use(&mut self, mut api: BattleApi<'_>) -> SkillResult<()> {
        let fighter_team = api.fighter().team;
        let multiplier = api.fighter().mixed_multiplier(0.3, 0.7);

        let ally_damage = (api.rng().gen_range(6..=8) as f32 * multiplier) as i32;
        let enemy_damage = (api.rng().gen_range(15..=19) as f32 * multiplier) as i32;

        api.emit_message(format!("**{}** invocou um poderoso terremoto na arena!", api.fighter().name));

        for index in api.battle().alive_fighters.clone() {
            let fighter = api.battle().get_fighter(index).clone();
            let dmg = if fighter.team == fighter_team {
                ally_damage
            } else {
                enemy_damage
            };

            let dmg = api.apply_damage(index, DamageSpecifier {
                culprit: api.fighter_index,
                kind: DamageKind::Physical,
                amount: dmg,
                accuracy: 90,
                balance_effectiveness: 15,
                effect: None,
            }).await;

            api.emit_random_message(&[
                format!("**{}** foi afetado por um terremoto e recebeu **{dmg}**!", fighter.name),
                format!("**{}** recebeu ferimentos pelo terremoto e recebeu **{dmg}**!", fighter.name),
                format!("**{}** foi ferido pelo terremoto e recebeu **{dmg}**!", fighter.name),
            ]);
        }

        Ok(())
    }
}
