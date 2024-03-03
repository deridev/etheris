use std::sync::Arc;

use etheris_commands::{parse_user_character, COMMANDS};
use etheris_common::config;
use etheris_data::emojis;
use etheris_database::{character_model::DeathCause, EtherisDatabase};
use etheris_discord::{
    application_command::CommandOptionValue,
    twilight_http::client::InteractionClient,
    twilight_model::{
        application::command::CommandOptionType,
        gateway::payload::incoming::InteractionCreate,
        id::{
            marker::{ApplicationMarker, GuildMarker},
            Id,
        },
    },
    ApiCommand, InteractionData, UserExtension,
};
use etheris_framework::*;
use etheris_framework::{
    watcher::Watcher, CommandBuilder, CommandContext, CommandOptionBuilder, EtherisClient,
};

pub async fn execute_command(
    interaction: Box<InteractionCreate>,
    client: Arc<EtherisClient>,
    watcher: Arc<Watcher>,
    database: Arc<EtherisDatabase>,
) -> anyhow::Result<()> {
    let data = interaction
        .data
        .clone()
        .and_then(|d| match d {
            InteractionData::ApplicationCommand(data) => Some(data),
            _ => None,
        })
        .ok_or(anyhow::anyhow!("Data not found"))?;

    let mut options = data.options.clone();

    let subcommand = match data.options.first() {
        Some(option) => match &option.value {
            CommandOptionValue::SubCommand(suboptions) => {
                options = suboptions.clone();
                Some(option.name.clone())
            }
            _ => None,
        },
        None => None,
    };

    let command_key = match subcommand {
        Some(subcommand) => format!("{} {subcommand}", data.name),
        None => data.name.to_owned(),
    };

    let mut ctx = CommandContext::new(
        client.clone(),
        Box::new(interaction.0),
        watcher,
        database,
        options,
    );
    let author = ctx.author().await?;
    let command = COMMANDS
        .get(&command_key)
        .ok_or(anyhow::anyhow!("Command not found"))?;

    let config = command.command_config();
    if config.character_required {
        let mut character = parse_user_character!(ctx, author);

        if let Some(death_info) = character.check_for_death() {
            ctx.db().characters().save(character.clone()).await?;
            let cause = match death_info.cause {
                DeathCause::Vitality => "Sem vitalidade".to_string(),
                DeathCause::KilledBy(killer_name) => format!("Morto(a) por {killer_name}"),
                DeathCause::Age => "Idade".to_string(),
            };

            ctx.reply(
                Response::new_user_reply(
                    &author,
                    format!(
                        "**o seu personagem morreu.**\nA causa da morte foi: `{cause}`.\n{}", 
                        format_args!("Seu personagem tinha **{} anos** e {} ◎ orbs na carteira.\nDescanse em paz, {}.", 
                            character.age(), character.orbs, character.name
                        ),
                    )
                ).add_emoji_prefix(format!("💸{}", emojis::STAMINA)),
            )
            .await?;
            return Ok(());
        }
    }

    if client.is_user_fighting(author.id).await {
        ctx.reply(
            Response::new_user_reply(&author, "termine sua batalha antes de usar um comando!")
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let result = command.run(ctx).await;
    if result.is_err() {
        eprintln!("[ERROR]\n{}", result.unwrap_err());
    }

    Ok(())
}

pub async fn register_commands(application_id: Id<ApplicationMarker>, client: Arc<EtherisClient>) {
    let commands: Vec<CommandBuilder> = {
        let mut parent_commands: Vec<(String, CommandBuilder)> = Vec::new();
        let mut commands = Vec::new();

        for (name, command) in COMMANDS.iter() {
            let splitted_name = name
                .split_ascii_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if splitted_name.len() == 1 {
                commands.push(command.build_command(application_id));
            } else {
                parent_commands.push((
                    splitted_name[0].clone(),
                    command.build_command(application_id),
                ));
            }
        }

        for (parent_name, command) in parent_commands.into_iter() {
            let parent_command = match commands
                .iter_mut()
                .find(|cmd| cmd.command.name == parent_name)
            {
                Some(command) => command,
                None => {
                    commands.push(CommandBuilder::new(application_id, &parent_name, "Group"));
                    commands
                        .iter_mut()
                        .find(|cmd| cmd.command.name == parent_name)
                        .unwrap()
                }
            };

            let builder = parent_command.clone();
            let subname = command
                .command
                .name
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            let subname = subname[1].to_owned();
            *parent_command = builder.add_option(
                CommandOptionBuilder::new(
                    subname,
                    command.command.description.clone(),
                    CommandOptionType::SubCommand,
                )
                .set_options(command.command.options),
            );
        }

        commands
    };

    let guild_id = match config::DEBUG {
        true => Some(Id::new(config::DEBUG_GUILD_ID)),
        false => None,
    };

    register_http_commands(
        guild_id,
        commands
            .into_iter()
            .map(|mut c| {
                if let Some(guild_id) = guild_id {
                    c = c.clone().set_guild_id(guild_id);
                }

                let build = c.build();
                println!(
                    "Registering command {}{}",
                    build.name,
                    if config::DEBUG { " (DEBUG)" } else { "" }
                );

                build
            })
            .collect::<Vec<ApiCommand>>()
            .as_slice(),
        client.http.interaction(application_id),
    )
    .await;
}

async fn register_http_commands<'a>(
    guild_id: Option<Id<GuildMarker>>,
    commands: &[ApiCommand],
    interaction: InteractionClient<'a>,
) {
    match guild_id {
        Some(guild_id) => {
            interaction
                .set_guild_commands(guild_id, commands)
                .await
                .expect("Failed to register guild commands");
        }
        _ => {
            interaction
                .set_global_commands(commands)
                .await
                .expect("Failed to register global commands");
        }
    };
}
