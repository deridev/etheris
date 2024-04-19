use etheris_database::character_model::BattleAction;
use etheris_framework::Response;

use crate::*;

pub async fn execute_action(action: BattleAction, api: &mut BattleApi<'_>) -> anyhow::Result<()> {
    match action {
        BattleAction::GiveUp => {
            api.fighter_mut().is_defeated = true;
            api.fighter_mut().defeated_by = None;

            api.emit_message(format!("**{}** desistiu!", api.fighter().name));
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
