use etheris_common::Probability;
use etheris_data::weapon::WeaponKind;
use rand::Rng;

use crate::list::prelude::*;

async fn weapon_stick(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(2..=6);

    let damage = api.rng().gen_range(8..=12);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let damage = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage,
                kind: DamageKind::Physical,
                balance_effectiveness: 5,
                accuracy: 95,
                effect: None,
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    api.emit_message(format!(
        "**{}** atacou **{}** com um graveto e causou **{damage}**",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

async fn weapon_knife(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(4..=8);

    let damage = api.rng().gen_range(10..=18);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let damage = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 3,
                accuracy: 80,
                effect: Some(Effect::new(EffectKind::Bleeding, 20, api.fighter_index)),
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    api.emit_message(format!(
        "**{}** cortou **{}** com uma faca e causou **{damage}**",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

async fn weapon_katana(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(8..=16);

    let damage_1 = api.rng().gen_range(4..=12);
    let damage_2 = api.rng().gen_range(4..=12);

    let damage_1 = base_damage + (damage_1 as f32 * api.fighter().weapon_multiplier()) as i32;
    let damage_2 = base_damage + (damage_2 as f32 * api.fighter().weapon_multiplier()) as i32;

    let damage_1 = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage_1,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 5,
                accuracy: 80,
                effect: Some(Effect::new(EffectKind::Bleeding, 25, api.fighter_index)),
            },
        )
        .await;

    let damage_2 = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage_2,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 5,
                accuracy: 60,
                effect: Some(Effect::new(EffectKind::Bleeding, 25, api.fighter_index)),
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    api.emit_message(format!(
        "**{}** desferiu dois cortes com a katana em **{}**, e causou **{damage_1}** no primeiro corte e **{damage_2}** no segundo!",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

async fn weapon_ethria_katana(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(4..=7);

    let damage_1 = api.rng().gen_range(6..=9);
    let damage_2 = api.rng().gen_range(6..=9);

    let damage_1 = base_damage + (damage_1 as f32 * api.fighter().weapon_multiplier()) as i32;
    let damage_2 = base_damage + (damage_2 as f32 * api.fighter().weapon_multiplier()) as i32;

    let damage_1 = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage_1,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 8,
                accuracy: 85,
                effect: Some(Effect::new(EffectKind::Bleeding, 29, api.fighter_index)),
            },
        )
        .await;

    let damage_2 = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage_2,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 7,
                accuracy: 60,
                effect: Some(Effect::new(EffectKind::Flaming, 37, api.fighter_index)),
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    api.emit_message(format!(
        "**{}** desferiu dois cortes com a katana flamejante em **{}**, e causou **{damage_1}** no primeiro corte e **{damage_2}** no segundo!",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

async fn weapon_spear(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(11..=12);
    let damage = api.rng().gen_range(12..=20);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let damage = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage,
                kind: DamageKind::PhysicalCut,
                balance_effectiveness: 10,
                accuracy: 60,
                effect: Some(Effect::new(EffectKind::Bleeding, 30, api.fighter_index)),
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    api.emit_message(format!(
        "**{}** perfurou **{}** com uma lança e causou **{damage}**",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

async fn weapon_bat(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(11..=15);
    let damage = api.rng().gen_range(15..=18);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let effect = if Probability::new(10).generate_random_bool() {
        Some(Effect::new(EffectKind::Bleeding, 60, api.fighter_index))
    } else {
        None
    };

    let damage = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage,
                kind: DamageKind::Physical,
                balance_effectiveness: if effect.is_some() { 20 } else { 10 },
                accuracy: 60,
                effect,
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    if effect.is_some() {
        api.emit_message(format!(
            "**{}** deu uma tacada MUITO FORTE no rosto de **{}** e causou **{damage}** e seu rosto começou a sangrar",
            api.fighter().name,
            target_name
        ))
    } else {
        api.emit_random_message(&[
            format!(
                "**{}** deu uma tacada em **{}** e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** bateu com um taco com força no peito de **{}** e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** deu uma tacada na cabeça de **{}** e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
        ]);
    }

    Ok(())
}

async fn weapon_umbrella(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(4..=15);
    let damage = api.rng().gen_range(10..=18);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let effect = if Probability::new(40).generate_random_bool() {
        Some(Effect::new(EffectKind::Bleeding, 60, api.fighter_index))
    } else {
        None
    };

    let damage = api
        .apply_damage(
            api.target_index,
            DamageSpecifier {
                culprit: api.fighter_index,
                amount: damage,
                kind: DamageKind::Physical,
                balance_effectiveness: if effect.is_some() { 20 } else { 10 },
                accuracy: 80,
                effect,
            },
        )
        .await;

    let target_name = api.target().name.to_owned();
    if effect.is_some() {
        api.emit_random_message(&[
            format!(
                "**{}** bateu com o guarda-chuva tão forte no rosto de **{}** que causou **{damage}** e seu rosto começou a sangrar",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** perfurou a barriga de **{}** com o guarda-chuva e causou **{damage}**!",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** bateu e cortou a pele de **{}** com o guarda-chuva e causou **{damage}**!",
                api.fighter().name,
                target_name
            )
        ])
    } else {
        api.emit_random_message(&[
            format!(
                "**{}** bateu em **{}** com o guarda-chuva e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** deu um estocada em **{}** com o guarda-chuva e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
            format!(
                "**{}** deu uma batida na cabeça de **{}** com o guarda-chuva e causou **{damage}**",
                api.fighter().name,
                target_name
            ),
        ]);
    }

    Ok(())
}

async fn weapon_scorpion_fang(mut api: BattleApi<'_>) -> anyhow::Result<()> {
    let base_damage = api.rng().gen_range(8..=20);
    let damage = api.rng().gen_range(10..=20);
    let damage = base_damage + (damage as f32 * api.fighter().weapon_multiplier()) as i32;

    let target = api.target();
    let target_name = target.name.clone();

    let damage = api
        .apply_damage(
            target.index,
            DamageSpecifier {
                culprit: api.fighter().index,
                kind: DamageKind::Physical,
                amount: damage,
                balance_effectiveness: 5,
                accuracy: 90,
                effect: Some(Effect::new(EffectKind::Poisoned, 7, api.fighter_index)),
            },
        )
        .await;

    api.emit_message(format!(
        "**{}** atacou **{}** com uma presa de escorpião e causou **{damage}**",
        api.fighter().name,
        target_name
    ));

    Ok(())
}

pub async fn execute_weapon_attack(
    api: BattleApi<'_>,
    weapon: FighterWeapon,
) -> anyhow::Result<()> {
    match weapon.kind {
        WeaponKind::Stick => weapon_stick(api).await,
        WeaponKind::Knife => weapon_knife(api).await,
        WeaponKind::Bat => weapon_bat(api).await,
        WeaponKind::Umbrella => weapon_umbrella(api).await,
        WeaponKind::Spear => weapon_spear(api).await,
        WeaponKind::Katana => weapon_katana(api).await,
        WeaponKind::ScorpionFang => weapon_scorpion_fang(api).await,
        WeaponKind::EthriaKatana => weapon_ethria_katana(api).await,
    }
}
