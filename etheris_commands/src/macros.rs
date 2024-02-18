/// Parses an Etheris character from the database or sends an error message to the ctx if the character does not exist.
/// This macro does not take ownership over ctx and user.
///
/// # Example
/// ```ignore
/// let author = ctx.author().await?;
/// let character: CharacterModel = parse_user_character!(ctx, author);
/// ```
#[macro_export]
macro_rules! parse_user_character {
    ($ctx:ident, $user:expr) => {
        {
            let Some(character) = $ctx.db().characters().get_by_user(&$user.id.to_string()).await? else {
                let error_message = if $user.id == $ctx.author_id() {
                    format!("você não possui um personagem registrado em Etheris! Registre com **/registrar** antes de usar esse comando.")
                } else {
                    format!("o usuário **{}** não possui um personagem registrado em Etheris!", $user.display_name())
                };

                return $ctx.reply(
                    Response::new_user_reply(&$user, error_message)
                        .error_response()
                ).await;
            };

            character
        }
    };
}

#[macro_export]
macro_rules! parse_city_from_character {
    ($ctx:ident, $character:expr) => {{
        let author = $ctx.author().await?;
        match $character.city_id {
            Some(city_id) => {
                let Some(city) = $ctx.db().cities().get_by_id(city_id).await? else {
                    $ctx.reply(
                        Response::new_user_reply(
                            &author,
                            "você precisa morar em uma cidade que existe!",
                        )
                        .add_emoji_prefix(emojis::ERROR),
                    )
                    .await?;
                    return Ok(());
                };

                city
            }
            None => {
                $ctx.reply(
                    Response::new_user_reply(
                        &author,
                        "o seu personagem precisa morar em uma cidade!",
                    )
                    .add_emoji_prefix(emojis::ERROR),
                )
                .await?;
                return Ok(());
            }
        }
    }};
}

#[macro_export]
macro_rules! parse_city_from_name {
    ($ctx:ident, $city_name:expr) => {{
        let author = $ctx.author().await?;
        match $city_name {
            Some(city_name) => {
                let Some(city) = $ctx.db().cities().get_by_name(&city_name).await? else {
                    $ctx.reply(
                        Response::new_user_reply(
                            &author,
                            "você precisa dizer o nome de uma cidade que existe!",
                        )
                        .add_emoji_prefix(emojis::ERROR),
                    )
                    .await?;
                    return Ok(());
                };

                city
            }
            None => {
                let character = parse_user_character!($ctx, author);
                let Some(city_id) = character.city_id else {
                    $ctx.reply(
                        Response::new_user_reply(
                            &author,
                            "você precisa dizer o nome da cidade que quer ver informações!",
                        )
                        .add_emoji_prefix(emojis::ERROR),
                    )
                    .await?;
                    return Ok(());
                };

                $ctx.db()
                    .cities()
                    .get_by_id(city_id)
                    .await?
                    .context("expected a valid city with this ID")?
            }
        }
    }};
}

#[macro_export]
macro_rules! verify_user_cooldown {
    ($ctx:expr, $user:ident, $cooldown:expr) => {
        if let Some(cooldown) = $ctx
            .db()
            .cooldowns()
            .get_user_cooldown($user.id.to_string(), $cooldown.into())
            .await?
        {
            let remaining = cooldown.ends_at.0 - chrono::Utc::now();
            if remaining.num_milliseconds() > 0 {
                $ctx.reply(
                    Response::new_user_reply(
                        &$user,
                        format!(
                            "você precisa esperar mais **{}** para fazer isso!",
                            $crate::util::format_duration(remaining)
                        ),
                    )
                    .add_emoji_prefix("⌛"),
                )
                .await?;
                return Ok(());
            }
        }
    };
}

pub use parse_user_character;
pub use verify_user_cooldown;
