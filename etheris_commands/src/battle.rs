use etheris_rpg::{Battle, BattleController, BattleSettings, FighterData};

use crate::prelude::*;

#[command("Enfrente alguém em uma batalha simples ou até o fim!")]
#[name("batalhar")]
#[character_required(true)]
pub async fn battle(
    mut ctx: CommandContext,
    #[rename("oponente")]
    #[description("O usuário que você quer desafiar")]
    opponent: User,
    #[rename("sério")]
    #[description("Se a batalha é séria e pode ser até a morte")]
    serious_battle: Option<bool>,
    #[rename("intrusos")]
    #[description("Número máximo de intrusos permitidos na luta")]
    intruders: Option<i64>,
) -> anyhow::Result<()> {
    let serious_battle = serious_battle.unwrap_or(false);
    let intruders = intruders.unwrap_or(0).clamp(0, 5) as u8;
    let author = ctx.author().await?;

    if opponent.id == author.id {
        ctx.send(
            Response::new_user_reply(&author, "você não pode lutar consigo mesmo!")
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let confirmation = ctx
        .helper()
        .create_confirmation(
            opponent.id,
            false,
            Response::new_user_reply(
                &opponent,
                format!(
                    "**{}** chamou você para um duelo {}. Você aceita?{}",
                    author.display_name(),
                    if serious_battle {
                        "**sério!** A batalha poderá ou não ser até a morte"
                    } else {
                        "casual"
                    },
                    if intruders > 0 {
                        format!("\nIntrusos permitidos na batalha: `{intruders}`")
                    } else {
                        String::new()
                    }
                ),
            )
            .add_emoji_prefix("❓"),
        )
        .await?;

    if !confirmation {
        return Ok(());
    };

    if ctx.client.is_user_fighting(author.id).await {
        ctx.send(
            Response::new_user_reply(&author, "você já está no meio de uma batalha!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    if ctx.client.is_user_fighting(opponent.id).await {
        ctx.send(
            Response::new_user_reply(&opponent, "você já está no meio de uma batalha!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let author_character = parse_user_character!(ctx, author);
    let opponent_character = parse_user_character!(ctx, opponent);

    if author_character.region != opponent_character.region {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você só pode lutar se estiver na mesma região do seu oponente!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let character_fighter =
        FighterData::new_from_character(0, &author_character, author, Default::default());
    let opponent_fighter =
        FighterData::new_from_character(1, &opponent_character, opponent, Default::default());

    let battle = Battle::new(
        author_character.region,
        BattleSettings {
            is_risking_life_allowed: true,
            has_consequences: serious_battle,
            casual: !serious_battle,
            max_intruders: intruders,
        },
        vec![character_fighter, opponent_fighter],
    )?;

    let mut controller = BattleController::new(battle, ctx);
    controller.run().await?;

    Ok(())
}
