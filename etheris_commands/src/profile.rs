use etheris_data::items::get_item_by_weapon;
use etheris_rpg::{
    list::get_boxed_skill_from_kind, pacts::list::get_boxed_pact_from_kind, Fighter, FighterData,
};

use crate::prelude::*;

#[command("Veja o perfil de um personagem em Etheris")]
#[name("perfil")]
#[character_required(true)]
pub async fn profile(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver o perfil")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let user = user.unwrap_or(ctx.author().await?);
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

    let pact_string = {
        let mut parts = vec![];
        for pact in character.pacts.iter() {
            let dyn_pact = get_boxed_pact_from_kind(pact.clone());
            let data = dyn_pact.data(&fighter);
            parts.push(format!("**{}**", data.name));
        }

        parts.join(", ")
    };

    let defeated_bosses_string = if character.defeated_bosses.is_empty() {
        String::new()
    } else {
        let mut parts = vec![];
        for defeated_boss in character.defeated_bosses.iter() {
            parts.push(format!("`{}`", defeated_boss.name()));
        }

        format!("\n**Chefes derrotados**: {}", parts.join(", "))
    };

    let embed = EmbedBuilder::new()
        .set_author(EmbedAuthor {
            name: format!("Personagem de {}", user.display_name()),
            icon_url: Some(user.avatar_url()),
        })
        .set_color(Color::BLURPLE)
        .set_description(format!(
            "{}\n{}\n**Habilidades**: {}{}\n**Nível de Poder**: `{} PL`{defeated_bosses_string}",
            character
                .personalities
                .iter()
                .map(|p| format!("**{p}**"))
                .collect::<Vec<_>>()
                .join(", "),
            if let Some(weapon) = character.weapon {
                format!(
                    "**Arma equipada**: {}",
                    get_item_by_weapon(weapon).display_name
                )
            } else {
                String::from("**Nenhuma arma equipada**")
            },
            character
                .skills
                .iter()
                .map(|s| format!("`{}`", get_boxed_skill_from_kind(s.clone()).data(&fighter).name))
                .collect::<Vec<_>>()
                .join(", "),
            if character.pacts.is_empty() {
                String::new()
            } else {
                format!("\n**Pactos**: {}", pact_string)
            },
            character.pl
        ))
        .add_field_with_emoji(
            Emoji::from_unicode("📄"),
            EmbedField {
                name: "Nome".into(),
                value: format!("```\n{}\n```", character.name),
                inline: false,
            },
        )
        .add_field_with_emoji(
            Emoji::from_unicode("🎈"),
            EmbedField {
                name: "Idade".into(),
                value: format!("{} anos", character.age()),
                inline: true,
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
        .add_field_with_emoji(
            character.region.kind().emoji_str(),
            EmbedField {
                name: "Região".into(),
                value: character.region.to_string(),
                inline: true,
            },
        )
        .add_field_with_emoji(
            Emoji::from_unicode("⚾"),
            EmbedField {
                name: "Pontos de Ação".into(),
                value: format!(
                    "{}{}",
                    if character.action_points <= character.max_action_points {
                        format!("**{}**/{}", character.action_points, character.max_action_points)
                    } else {
                        bold(&character.action_points.to_string())
                    },
                    if character.action_points < character.max_action_points {
                        format!("\nRecarrega em: `{}`",
                        crate::util::format_duration(
                            (character.last_refill.0
                                + chrono::Duration::try_minutes(character.refill_minutes as i64).unwrap())
                                - chrono::Utc::now()
                        ))
                    } else {
                        String::new()
                    }
                ),
                inline: true,
            },
        )
        .add_field_with_emoji(
            Emoji::from_unicode("🌀"),
            EmbedField {
                name: "Experiência".into(),
                value: format!(
                    "Vida: `{} XP`\nForça: `{} XP`\nInteligência: `{} XP`\nConhecimento: **{} pontos**",
                    character.health_xp, character.strength_xp, character.intelligence_xp, character.knowledge_points
                ),
                inline: true,
            },
        )
        .add_field_with_emoji(Emoji::from_unicode("💪"), EmbedField {
            name: "Atributos".into(),
            value: format!("{} **Resistência**: **{}**/{}\n{} **Vitalidade**: {}/{}\n{} **Ether**: {}/{}", emojis::RESISTANCE, character.stats.resistance.value, character.stats.resistance.max, emojis::VITALITY, character.stats.vitality.value, character.stats.vitality.max, emojis::ETHER, character.stats.ether.value, character.stats.ether.max),
            inline: true,
        })
        .set_image(if let Some(attachment) = &attachment {
            format!("attachment://{}", attachment.filename)
        } else {
            user.avatar_url()
        })
        .add_footer_text(
            "Use /habilidades para ver mais informações sobre habilidades do personagem",
        )
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
