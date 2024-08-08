use twilight_model::channel::message::component::ButtonStyle;

use crate::prelude::*;

#[command("Servidor oficial de Etheris")]
#[name("servidor")]
#[character_required(true)]
pub async fn guild(mut ctx: CommandContext) -> anyhow::Result<()> {
    let embed = EmbedBuilder::new_common()
        .set_author_to_user(&ctx.author().await?)
        .set_color(Color::BLURPLE)
        .set_description("## Servidor de Etheris! 🌀\nEntre no servidor oficial de Etheris para receber suporte, ajuda e novidades!");

    let action_row = ActionRowBuilder::new().add_button(
        ButtonBuilder::new()
            .set_label("Entrar")
            .set_url("https://discord.gg/74bexXwy")
            .set_style(ButtonStyle::Link),
    );

    ctx.send(Response::from(embed).set_components(vec![action_row]))
        .await?;

    Ok(())
}
