use etheris_data::{
    appearance::Cosmetic,
    items::{get_item_by_weapon, Item, ALL_ITEMS},
};

use crate::prelude::*;

#[command("Desequipe todos os cosméticos do seu personagem!")]
#[name("desequipar")]
#[character_required(true)]
pub async fn unequip(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);

    let face_item = get_cosmetic_item(&character.appearance.face_cosmetic);
    let head_item = get_cosmetic_item(&character.appearance.head_cosmetic);
    let weapon_item = character.weapon.map(get_item_by_weapon);

    if let Some(item) = face_item {
        character.add_item(item, 1, Some(item.default_values.into()));
    }

    if let Some(item) = head_item {
        character.add_item(item, 1, Some(item.default_values.into()));
    }

    if let Some(item) = weapon_item {
        character.add_item(item, 1, Some(item.default_values.into()));
    }

    character.appearance.face_cosmetic = Cosmetic::default();
    character.appearance.head_cosmetic = Cosmetic::default();
    character.weapon = None;
    ctx.db().characters().save(character).await?;

    ctx.reply(
        Response::new_user_reply(
            &author,
            "você desequipou com sucesso tudo que seu personagem vestia e usava como arma! Você pode equipar novamente com **/equipar**.",
        )
        .add_emoji_prefix(emojis::SUCCESS),
    )
    .await?;

    Ok(())
}

fn get_cosmetic_item(cosmetic: &Cosmetic) -> Option<Item> {
    ALL_ITEMS
        .iter()
        .find(|item| {
            if let Some(item_cosmetic) = item.cosmetic_properties {
                if item_cosmetic.cosmetic_identifier == cosmetic.identifier {
                    return true;
                }
            }

            false
        })
        .copied()
}
