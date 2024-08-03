use twilight_model::channel::message::component::ButtonStyle;

use crate::prelude::*;

#[command("Convide Etheris para seu servidor!")]
#[name("convidar")]
#[character_required(true)]
pub async fn invite(mut ctx: CommandContext) -> anyhow::Result<()> {
    let embed = EmbedBuilder::new_common()
        .set_author_to_user(&ctx.author().await?)
        .set_color(Color::BLURPLE)
        .set_description("## Convide Etheris para seu servidor!");

    let action_row = ActionRowBuilder::new().add_button(
        ButtonBuilder::new()
            .set_label("Convidar")
            .set_url("https://top.gg/bot/1195306808860999680")
            .set_style(ButtonStyle::Link),
    );

    ctx.send(Response::from(embed).set_components(vec![action_row]))
        .await?;

    Ok(())
}
