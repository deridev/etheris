use std::time::Duration;

use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::watcher::WatcherOptions;

use crate::prelude::*;

#[command("Viaje para outra região em Etheris!")]
#[name("viajar")]
#[character_required(true)]
pub async fn travel(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    let is_travel_free = character.region.city().is_some();

    let character_image = character.create_image_bufer().unwrap_or_default();
    let attachment = DiscordAttachment::from_bytes("image.png".to_owned(), character_image, 1);

    let neighbors = character.region.neighbors();

    let mut regions = vec![];
    let mut embed = EmbedBuilder::new_common()
        .set_color(Color::ORANGE)
        .set_author(EmbedAuthor {
            name: format!("Viagem de {}", character.name),
            icon_url: Some(author.avatar_url()),
        })
        .set_thumbnail(format!("attachment://{}", attachment.filename))
        .set_description(format!("## Você tem {} ◎ {}", character.orbs, emojis::ORB))
        .add_footer_text("Viajar custa 5 pontos de ação");

    macro_rules! add_region {
        ($dir:ident, $dir_name:expr) => {
            if let Some(region) = neighbors.$dir {
                embed = embed.add_field(EmbedField {
                    name: format!("{} {}", region.kind().emoji_str(), region),
                    value: format!(
                        "**{}**\n**{} {} ◎**\n{}",
                        region.kind(),
                        emojis::ORB,
                        if is_travel_free {
                            0
                        } else {
                            region.data().travel_price
                        },
                        $dir_name
                    ),
                    inline: true,
                });

                regions.push(region);
            }
        };
    }

    add_region!(up, "Ao Norte");
    add_region!(left, "Ao Oeste");
    add_region!(right, "Ao Leste");
    add_region!(down, "Ao Sul");

    let buttons = regions
        .iter()
        .map(|region| {
            ButtonBuilder::new()
                .set_custom_id(region.to_string())
                .set_label(region.to_string())
                .set_disabled(!is_travel_free && region.data().travel_price > character.orbs)
        })
        .collect::<Vec<_>>();
    let row = ActionRowBuilder::new().add_buttons(buttons.clone());

    let message = ctx
        .send(
            Response::new_user_reply(&author, "escolha para onde você quer viajar:")
                .add_embed(embed)
                .set_components(vec![row]),
        )
        .await?;

    let Ok(Some(collected)) = ctx
        .watcher
        .await_single_component(
            message.id,
            move |interaction| interaction.author_id() == Some(author.id),
            WatcherOptions {
                timeout: Duration::from_secs(60),
            },
        )
        .await
    else {
        return Ok(());
    };

    let data = collected.parse_message_component_data()?;

    let mut character = parse_user_character!(ctx, author);
    let mut ctx = CommandContext::from_with_interaction(&ctx, Box::new(collected));

    let buttons = buttons
        .into_iter()
        .map(|b| {
            let custom_id = b.data.custom_id.clone().unwrap_or_default();
            b.set_disabled(true)
                .set_style(if custom_id == data.custom_id {
                    ButtonStyle::Success
                } else {
                    ButtonStyle::Secondary
                })
        })
        .collect::<Vec<_>>();

    ctx.update_message(
        Response::default().set_components(vec![ActionRowBuilder::new().add_buttons(buttons)]),
    )
    .await
    .ok();

    let Some(region) = regions.iter().find(|r| r.to_string() == data.custom_id) else {
        return Ok(());
    };

    if character.action_points < 5 {
        ctx.send(
            Response::new_user_reply(
                &author,
                format!(
                    "você precisa de **5 pontos de ação** para viajar até **{}**!",
                    region
                ),
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    if !is_travel_free && character.orbs < region.data().travel_price {
        ctx.send(
            Response::new_user_reply(
                &author,
                format!(
                    "você precisa de **{} {} orbs ** para viajar até **{}**!",
                    emojis::ORB,
                    region.data().travel_price,
                    region
                ),
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    character.action_points -= 5;
    if !is_travel_free {
        character.remove_orbs(region.data().travel_price);
    }
    character.travel_to(*region);

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(&author, format!("você viajou até **{}**!", region))
            .add_emoji_prefix(format!("{} {}", region.kind().emoji_str(), emojis::SUCCESS)),
    )
    .await?;

    Ok(())
}
