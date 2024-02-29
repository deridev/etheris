use std::collections::HashSet;

use etheris_rpg::events::EventController;
use once_cell::sync::Lazy;

use crate::prelude::*;

static mut USERS_EXPLORING: Lazy<HashSet<Id<UserMarker>>> = Lazy::new(HashSet::new);

#[command("Explore sua região em busca de itens, inimigos ou outras coisas!")]
#[name("explorar")]
#[character_required(true)]
pub async fn explore(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let author_id = author.id;
    let character = parse_user_character!(ctx, author);
    if unsafe { USERS_EXPLORING.contains(&author_id) } {
        ctx.send(
            Response::new_user_reply(&author, "você já está explorando! Finalize sua exploração antes de usar esse comando novamente.")
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    if character.action_points < 1 {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você precisa de **1 ponto de ação** para explorar! Veja quando seus pontos de ações recarregam em **/perfil**.\n❓ **Dica**: estudar (`/estudar`) faz seus pontos de ação aumentarem e recarregarem mais rápido!"
            )
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let Some(event) = EventController::pick_event(character) else {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você não encontrou nenhum evento! Tente viajar para outro lugar para encontrar mais coisas interessantes.",
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        ).await?;

        return Ok(());
    };

    unsafe {
        USERS_EXPLORING.insert(author_id);
    }

    match EventController::new(author.clone(), ctx.clone(), vec![event])
        .execute()
        .await
    {
        Ok(_) => {
            unsafe {
                USERS_EXPLORING.remove(&author_id);
            }

            let mut character = parse_user_character!(ctx, author);
            character.action_points = character.action_points.saturating_sub(1);
            ctx.db().characters().save(character).await?;
            return Ok(());
        }
        Err(e) => {
            unsafe {
                USERS_EXPLORING.remove(&author_id);
            }
            return Err(e);
        }
    };
}
