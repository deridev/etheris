use etheris_data::{
    items::{get_item, get_item_by_name},
    util::{translate, untranslate},
};
use etheris_database::bson::oid::ObjectId;
use etheris_util::{
    character_image::generate_character_image_buffer, generate_random_character_appearance,
};

use crate::prelude::*;

type IdString = String;

#[derive(Debug, Clone)]
pub enum Command {
    Help,
    ResetCache(Option<IdString>),
    ResetCooldowns(Option<IdString>),
    Reskin(IdString),
    AddOrbs(IdString, i32),
    RemoveOrbs(IdString, i32),
    AddItem(IdString, String, i32),
    RemoveItem(IdString, String, i32),
    Translate(String),
    Untranslate(String),
    GenerateSkin,
    AddAP(IdString, u32),
    AddAPAll(u32),
}

impl Command {
    pub const HELP_EXAMPLES: &'static [&'static str] = &[
        "help",
        "reset cache [id]",
        "reset cooldowns [id]",
        "reskin <id>",
        "add orbs <id> [quantity]",
        "remove orbs <id> [quantity]",
        "add item <id> <item> [quantity]",
        "remove item <id> <item> [quantity]",
        "translate <text>",
        "untranslate <text>",
        "generate skin",
        "add ap <id> [quantity]",
        "add ap_all [quantity]",
    ];

    pub fn parse(input: &str) -> Option<Command> {
        let mut splitted = input.split(' ');

        let command = splitted.next()?.to_lowercase();

        match command.as_str() {
            "help" => Some(Command::Help),
            "translate" => Some(Command::Translate(splitted.collect::<Vec<_>>().join(" "))),
            "untranslate" => Some(Command::Untranslate(splitted.collect::<Vec<_>>().join(" "))),
            "reset" => {
                let subcommand = splitted.next()?.to_lowercase();
                match subcommand.as_str() {
                    "cache" => {
                        let id = splitted.next().map(str::to_owned);
                        Some(Command::ResetCache(id))
                    }
                    "cooldowns" | "cd" => {
                        let id = splitted.next().map(str::to_owned);
                        Some(Command::ResetCooldowns(id))
                    }
                    _ => None,
                }
            }
            "reskin" => {
                let id = splitted.next().map(str::to_owned)?;
                Some(Command::Reskin(id))
            }
            "add" | "remove" => {
                let subcommand = splitted.next()?.to_lowercase();

                match subcommand.as_str() {
                    "orbs" => {
                        let id = splitted.next()?.to_lowercase();
                        let quantity = splitted.next().unwrap_or("1").parse::<i32>().ok()?;

                        match command.as_str() {
                            "add" => Some(Command::AddOrbs(id, quantity)),
                            "remove" => Some(Command::RemoveOrbs(id, quantity)),
                            _ => None,
                        }
                    }
                    "item" => {
                        let id = splitted.next()?.to_lowercase();
                        let item = splitted.next()?.to_owned();
                        let quantity = splitted.next().unwrap_or("1").parse::<i32>().ok()?;

                        match command.as_str() {
                            "add" => Some(Command::AddItem(id, item, quantity)),
                            "remove" => Some(Command::RemoveItem(id, item, quantity)),
                            _ => None,
                        }
                    }
                    "ap" => {
                        let id = splitted.next()?.to_lowercase();
                        let quantity = splitted.next().unwrap_or("1").parse::<u32>().ok()?;
                        Some(Command::AddAP(id, quantity))
                    }
                    "ap_all" => {
                        let quantity = splitted.next().unwrap_or("1").parse::<u32>().ok()?;
                        Some(Command::AddAPAll(quantity))
                    }
                    _ => None,
                }
            }
            "generate" => {
                let subcommand = splitted.next()?.to_lowercase();

                match subcommand.as_str() {
                    "skin" => Some(Command::GenerateSkin),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[command("Comando restrito.")]
#[name("adm")]
#[character_required(true)]
pub async fn adm(
    mut ctx: CommandContext,
    #[rename("comando")]
    #[description("Comando a ser executado")]
    command: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    if author.id.get() != 518830049949122571 {
        ctx.reply(
            Response::new_user_reply(&author, "você não tem permissão para usar esse comando!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let Some(command) = Command::parse(&command) else {
        ctx.reply(
            Response::new_user_reply(&author, "comando inválido!").add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    };

    macro_rules! parse_character {
        ($id:expr) => {{
            let id = match $id.as_str() {
                "self" => author.id.to_string(),
                _ => $id,
            };

            match ctx.db().characters().get_by_user(&id).await? {
                Some(character) => Some(character),
                None => {
                    ctx.db()
                        .characters()
                        .get_by_id(ObjectId::parse_str(id)?)
                        .await?
                }
            }
        }};
    }

    match command {
        Command::Help => {
            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "exemplos de comandos:\n```\n{}\n```",
                    Command::HELP_EXAMPLES.join("\n")
                ),
            ))
            .await?;
        }
        Command::Translate(text) => {
            let output = translate(&text);
            ctx.reply(Response::new_user_reply(&author, format!("```{output}```")))
                .await?;
        }
        Command::Untranslate(text) => {
            let output = untranslate(&text);
            ctx.reply(Response::new_user_reply(&author, format!("```{output}```")))
                .await?;
        }
        Command::ResetCache(id) => {
            let character = parse_character!(id.clone().unwrap_or(author.id.to_string()))
                .context("character not found")?;
            ctx.db().characters().remove_from_cache(&character);
            ctx.reply(Response::new_user_reply(
                &author,
                format!("você resetou o cache de {} com sucesso.", character.name),
            ))
            .await?;
        }
        Command::ResetCooldowns(id) => {
            let user = ctx
                .client
                .get_user(Id::new(
                    id.clone().unwrap_or(author.id.to_string()).parse()?,
                ))
                .await?;
            ctx.db()
                .cooldowns()
                .delete_all_user_cooldowns(user.id.to_string())
                .await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você resetou os cooldowns de {} com sucesso.",
                    user.display_name()
                ),
            ))
            .await?;
        }
        Command::Reskin(id) => {
            let mut character = parse_character!(id).context("character not found")?;
            character.appearance = generate_random_character_appearance();

            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você regerou a aparência do personagem **{}** com sucesso.",
                    character.name
                ),
            ))
            .await?;
        }
        Command::AddOrbs(id, quantity) => {
            let mut character = parse_character!(id).context("character not found")?;

            character.add_orbs(quantity.clamp(1, i32::MAX) as i64);
            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você adicionou **{} ◎** ao personagem {} com sucesso.",
                    quantity, character.name
                ),
            ))
            .await?;
        }
        Command::RemoveOrbs(id, quantity) => {
            let mut character = parse_character!(id).context("character not found")?;

            character.remove_orbs(quantity.clamp(1, i32::MAX) as i64);
            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você removeu **{} ◎** ao personagem {} com sucesso.",
                    quantity, character.name
                ),
            ))
            .await?;
        }
        Command::AddItem(id, item, quantity) => {
            let mut character = parse_character!(id).context("character not found")?;
            let item = match get_item(&item) {
                Some(item) => item,
                None => get_item_by_name(&item).context("item not found")?,
            };

            character.add_item(item, quantity.clamp(1, i32::MAX) as usize, None);
            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você adicionou **{}x {}** ao personagem {} com sucesso.",
                    quantity, item.display_name, character.name
                ),
            ))
            .await?;
        }
        Command::RemoveItem(id, item, quantity) => {
            let mut character = parse_character!(id).context("character not found")?;
            let item = match get_item(&item) {
                Some(item) => item,
                None => get_item_by_name(&item).context("item not found")?,
            };

            character.remove_item(item, quantity.clamp(1, i32::MAX) as usize);
            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você removeu **{}x {}** ao personagem {} com sucesso.",
                    quantity, item.display_name, character.name
                ),
            ))
            .await?;
        }
        Command::GenerateSkin => {
            let image = generate_character_image_buffer(&generate_random_character_appearance());
            let attachment = DiscordAttachment::from_bytes("image.png".to_owned(), image, 1);

            let embed = EmbedBuilder::new_common()
                .set_color(Color::LIGHT_CYAN)
                .set_image(format!("attachment://{}", attachment.filename));

            ctx.reply(Response::from(embed).set_attachments(vec![attachment]))
                .await?;
        }
        Command::AddAP(id, quantity) => {
            let mut character = parse_character!(id).context("character not found")?;

            character.action_points += quantity;
            ctx.db().characters().save(character.clone()).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você adicionou **{} AP** ao personagem {} com sucesso.",
                    quantity, character.name
                ),
            ))
            .await?;
        }
        Command::AddAPAll(quantity) => {
            let quantity = quantity.clamp(1, u32::MAX);
            ctx.db().characters().add_ap_to_everyone(quantity).await?;

            ctx.reply(Response::new_user_reply(
                &author,
                format!(
                    "você adicionou **{} AP** a todos os personagens com sucesso.",
                    quantity
                ),
            ))
            .await?;
        }
    }

    Ok(())
}
