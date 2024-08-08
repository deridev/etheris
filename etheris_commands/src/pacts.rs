use etheris_rpg::{pacts::list::get_boxed_pact_from_kind, Fighter, FighterData};

use crate::prelude::*;

#[command("Veja os pactos habilidades de um personagem em Etheris")]
#[name("pactos")]
#[character_required(true)]
pub async fn pacts(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver os pactos")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let user = user.unwrap_or(author.clone());
    let character = parse_user_character!(ctx, user);
    let fighter = Fighter::new(
        0,
        Default::default(),
        Default::default(),
        FighterData::new_from_character(0, &character, user.clone(), Default::default()),
    );

    let image = character.create_image_bufer();
    let attachment =
        image.map(|image| DiscordAttachment::from_bytes("image.png".to_owned(), image, 1));

    let pacts = character
        .pacts
        .iter()
        .map(|kind| get_boxed_pact_from_kind(kind.clone()))
        .collect::<Vec<_>>();

    if pacts.is_empty() {
        let name = if user.id == author.id {
            "você".to_string()
        } else {
            user.display_name()
        };

        ctx.reply(
            Response::new_user_reply(&user, format!("**{}** não possui nenhum pacto!", name))
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let description = {
        let mut parts = vec![];
        for pact in pacts.iter() {
            let data = pact.data(&fighter);
            let kind = pact.kind();
            let rarity = kind.rarity();
            parts.push(format!(
                "### **{}**\n**{}**\n{}\n\n*{}*",
                data.name,
                rarity.name(),
                data.description,
                data.explanation
            ));
        }

        parts.join("\n")
    };

    let embed = EmbedBuilder::new()
        .set_author(EmbedAuthor {
            name: format!("Pactos do personagem {}", character.name),
            icon_url: Some(user.avatar_url()),
        })
        .set_color(Color::BLURPLE)
        .set_description(description)
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
