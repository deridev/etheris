use twilight_model::channel::message::component::ButtonStyle;

use crate::prelude::*;

#[command("Ao votar, seu personagem é beneficiado!")]
#[name("votar")]
#[character_required(true)]
pub async fn vote(mut ctx: CommandContext) -> anyhow::Result<()> {
    let embed = EmbedBuilder::new_common()
        .set_author_to_user(&ctx.author().await?)
        .set_color(Color::BLURPLE)
        .set_description("## Vote em Etheris! ❤️\nVotando em Etheris no site **Top.gg**, além de você ajudar o bot a crescer, seu personagem recupera todos os pontos de ação e descansa completamente. De graça!\nVocê pode votar 1 vez a cada 12 horas.");

    let action_row = ActionRowBuilder::new().add_button(
        ButtonBuilder::new()
            .set_label("Votar")
            .set_url("https://top.gg/bot/1195306808860999680")
            .set_style(ButtonStyle::Link),
    );

    ctx.send(Response::from(embed).set_components(vec![action_row]))
        .await?;

    Ok(())
}
