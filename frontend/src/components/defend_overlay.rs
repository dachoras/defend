//! Game status overlays (defeat) for Defend.

use crate::components::defend_logic::GameStatus;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub status: GameStatus,
    pub score: u32,
    pub wave: u32,
    pub on_restart: Callback<()>,
}

#[function_component(DefendOverlay)]
pub fn defend_overlay(props: &Props) -> Html {
    let locale = use_context::<LocaleContext>().unwrap_or_default();

    let restart_click = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_| on_restart.emit(()))
    };

    match props.status {
        GameStatus::Lost => {
            html! {
                <div class="game-overlay defeat glassmorphic">
                    <h2 class="outcome-title compromised">{ locale.t("game_over") }</h2>
                    <p class="stat-line">{ format!("FINAL SCORE: {}", props.score) }</p>
                    <p class="stat-line">{ format!("WAVE REACHED: {}", props.wave) }</p>
                    <button class="btn-restart" onclick={restart_click}>
                        { locale.t("play_again") }
                    </button>
                </div>
            }
        }
        _ => html! {},
    }
}
