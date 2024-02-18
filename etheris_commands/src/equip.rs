use etheris_data::{
    appearance::{FACE_COSMETICS, HEAD_COSMETICS},
    items::{CosmeticKind, Item},
    weapon::WeaponKind,
};

use crate::prelude::*;

#[command("Equipe no seu personagem um item cosmético!")]
#[name("equipar")]
#[character_required(true)]
pub async fn equip(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer vestir")]
    item: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let Some(item) = items::get_item_by_name(&item) else {
        ctx.reply(Response::new_user_reply(
            &author,
            format!("nenhum item chamado **{}** foi encontrado! Utilize **/inventário** para ver os seus itens.", item)
        ).add_emoji_prefix(emojis::ERROR)
        ).await?;

        return Ok(());
    };

    let mut character = parse_user_character!(ctx, author);
    if !character.has_item(&item, 1) {
        ctx.reply(
            Response::new_user_reply(&author, "você não possui esse item!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    if let Some(weapon) = item.weapon {
        equip_weapon(&mut ctx, weapon, item).await?;
        return Ok(());
    }

    let Some(cosmetic_properties) = item.cosmetic_properties else {
        ctx.reply(
            Response::new_user_reply(&author, "esse item não pode ser equipado!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;

        return Ok(());
    };

    let cosmetic = match cosmetic_properties.kind {
        CosmeticKind::Face => FACE_COSMETICS
            .iter()
            .find(|c| c.name == cosmetic_properties.cosmetic_identifier),
        CosmeticKind::Head => HEAD_COSMETICS
            .iter()
            .find(|c| c.name == cosmetic_properties.cosmetic_identifier),
    };

    let Some(cosmetic) = cosmetic else {
        ctx.reply(
            Response::new_user_reply(&author,
                format!(
                    "algo deu errado ao encontrar o cosmético desse item. Esse é um erro Etheris! Avise o desenvolvedor Etheris, por favor. Envie para ele o ID do cosmético abaixo:\n`{:?}-{}`", 
                    cosmetic_properties.kind, cosmetic_properties.cosmetic_identifier
                ))
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;

        return Ok(());
    };

    match cosmetic_properties.kind {
        CosmeticKind::Face => character.appearance.face_cosmetic = (*cosmetic).into(),
        CosmeticKind::Head => character.appearance.head_cosmetic = (*cosmetic).into(),
    };

    character.remove_item(item, 1);
    ctx.db().characters().save(character).await?;

    ctx.reply(
        Response::new_user_reply(
            &author,
            format!("você equipou com sucesso o item **{}**!", item.display_name),
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    Ok(())
}

pub async fn equip_weapon(
    ctx: &mut CommandContext,
    weapon: WeaponKind,
    item: Item,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    if character.weapon.is_some() {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "desequipe a arma que você já está equipando com **/desequipar** antes!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    character.remove_item(item, 1);
    character.weapon = Some(weapon);
    ctx.db().characters().save(character).await?;

    ctx.reply(
        Response::new_user_reply(
            &author,
            format!("você equipou com sucesso a arma **{}**!", item.display_name),
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    Ok(())
}
