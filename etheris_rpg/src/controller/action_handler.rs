use etheris_database::character_model::BattleAction;
use etheris_framework::Response;
use rand::prelude::SliceRandom;

use crate::*;

pub async fn execute_action(action: BattleAction, api: &mut BattleApi<'_>) -> anyhow::Result<()> {
    match action {
        BattleAction::GiveUp => {
            api.fighter_mut().flags.insert(FighterFlags::GAVE_UP);
            api.emit_message(format!("**{}** desistiu!", api.fighter().name));

            // Guard against letting all fighters give up
            let alive_fighters = api.battle().alive_fighters.clone();
            let alive_fighters = alive_fighters
                .iter()
                .map(|index| api.battle().get_fighter(*index).clone())
                .collect::<Vec<_>>();

            if !alive_fighters.is_empty()
                && alive_fighters
                    .iter()
                    .all(|f| f.flags.contains(FighterFlags::GAVE_UP))
            {
                let random_fighter = alive_fighters.choose(&mut api.rng()).unwrap();
                let fighter = api.battle_mut().get_fighter_mut(random_fighter.index);
                fighter.flags.remove(FighterFlags::GAVE_UP);
            }
            Ok(())
        }
        BattleAction::ControlPower => {
            let power_choosen = match api.fighter().user.clone() {
                Some(user) => {
                    let response = Response::new_user_reply(
                        &user,
                        format!(
                            "escreva quando poder você quer liberar (**0** até **{}**):",
                            (api.fighter().potential * 100.0).round() as i32
                        ),
                    );

                    let Ok(input) = api_input::input_number(
                        api,
                        response,
                        (0.0, (api.fighter().potential * 100.0).round()),
                    )
                    .await
                    else {
                        api.emit_message(format!(
                            "**{}** não conseguiu controlar seu poder a tempo!",
                            api.fighter().name
                        ));
                        return Ok(());
                    };

                    input / 100.0
                }
                None => api.fighter().potential,
            };

            api.fighter_mut().power = power_choosen;
            api.fighter_mut().recalculate_pl();
            api.emit_message(format!(
                "**{}** controlou seu poder e mudou para **{}%**!",
                api.fighter().name,
                (power_choosen * 100.0).round() as i32
            ));
            Ok(())
        }
    }
}
