use crate::prelude::*;

#[command("Veja a carteira de um personagem em Etheris")]
#[name("carteira")]
#[character_required(true)]
pub async fn wallet(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver a carteira")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let user = user.unwrap_or(ctx.author().await?);
    let character = parse_user_character!(ctx, user);

    let image = character.create_image_bufer();
    let attachment =
        image.map(|image| DiscordAttachment::from_bytes("image.png".to_owned(), image, 1));

    let embed = EmbedBuilder::new()
        .set_author(EmbedAuthor {
            name: format!("Personagem de {}", user.display_name()),
            icon_url: Some(user.avatar_url()),
        })
        .set_color(Color::LIGHT_CYAN)
        .add_field_with_emoji(
            Emoji::from_unicode("📄"),
            EmbedField {
                name: "Nome".into(),
                value: character.name.to_owned(),
                inline: false,
            },
        )
        .add_field_with_emoji(
            emojis::ORB,
            EmbedField {
                name: "Orbs".into(),
                value: format!("{} ◎", character.orbs.to_readable_string()),
                inline: true,
            },
        )
        .set_thumbnail(if let Some(attachment) = &attachment {
            format!("attachment://{}", attachment.filename)
        } else {
            user.avatar_url()
        })
        .set_current_timestamp();

    ctx.reply(
        Response::from(embed).set_attachments(if let Some(attachment) = attachment {
            vec![attachment]
        } else {
            vec![]
        }),
    )
    .await?;

    Ok(())
}
