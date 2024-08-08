use std::net::SocketAddr;
use std::sync::Arc;

use etheris_common::Color;
use etheris_data::items;
use etheris_database::EtherisDatabase;
use etheris_discord::twilight_model::id::Id;
use etheris_discord::EmbedBuilder;
use etheris_framework::EtherisClient;
use serde::{Deserialize, Serialize};
use warp::http::Response as WarpResponse;
use warp::reply::Response;
use warp::Filter;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct VoteObject {
    pub user: String,
}

pub fn spawn_vote_thread(client: Arc<EtherisClient>, database: Arc<EtherisDatabase>) {
    let client = client.clone();
    let db = database.clone();
    tokio::spawn(async move {
        let root_route = warp::path::end()
            .map(|| warp::reply::html("This is the API for Etheris. Get out of here."));

        let votes_route = warp::path("votes")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |payload: serde_json::Value| (payload, client.clone(), db.clone()))
            .and_then(
                |(payload, client, database): (
                    serde_json::Value,
                    Arc<EtherisClient>,
                    Arc<EtherisDatabase>,
                )| async move {
                    println!("\n[VOTE OBJECT]\n{:?}", payload);

                    let vote_object: Option<VoteObject> =
                        serde_json::from_value(payload.clone()).ok();
                    if let Some(vote_object) = vote_object {
                        vote(vote_object, client, database).await.ok();
                    }

                    Ok::<Response, warp::Rejection>(
                        WarpResponse::builder()
                            .status(200)
                            .body("OK".into())
                            .expect("Building WarpResponse failed"),
                    )
                },
            );

        let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

        let routes = root_route.or(votes_route);
        warp::serve(routes).run(addr).await;
    });
    println!("Top.gg listener started on port 8000");
}

async fn vote(
    object: VoteObject,
    client: Arc<EtherisClient>,
    database: Arc<EtherisDatabase>,
) -> anyhow::Result<()> {
    let uid = object.user.parse::<u64>()?;
    let uid = Id::new(uid);

    let user = client.get_user(uid).await?;
    let character = database
        .characters()
        .get_by_user(&user.id.to_string())
        .await?;

    let Some(mut character) = character else {
        return Ok(());
    };

    character.action_points = if character.action_points < character.max_action_points {
        character.max_action_points
    } else {
        character.action_points
    };

    character.stats.resistance.value = character.stats.resistance.max;
    character.stats.vitality.value = character.stats.vitality.max;
    character.stats.ether.value = character.stats.ether.max;
    character.add_item(items::special::INTERNAL_KEY, 1, None);

    database.characters().save(character).await?;

    let dm_channel = client
        .http
        .create_private_channel(user.id)
        .await?
        .model()
        .await?;

    let embed = EmbedBuilder::new_common()
        .set_color(Color::GREEN)
        .set_author_to_user(&user)
        .set_description(format!("## Obrigado por votar! 💫\nPor ter votado em Etheris, seu personagem descansou completamente e ganhou **1x {} {}**. Aproveite Etheris!", items::special::INTERNAL_KEY.display_name, items::special::INTERNAL_KEY.emoji))
        .add_footer_text("Em 12 horas você poderá votar novamente. 👀");
    client
        .http
        .create_message(dm_channel.id)
        .embeds(&[embed.build()])?
        .await?;

    println!("{} voted!", user.name);

    Ok(())
}
