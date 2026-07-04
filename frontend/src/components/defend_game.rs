//! Main Defend gameplay container component.

use crate::components::defend_board::DefendBoard;
use crate::components::defend_controls::DefendControls;
use crate::components::defend_logic::{GameState, GameStatus};
use crate::components::defend_overlay::DefendOverlay;
use crate::i18n::LocaleContext;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_status: Callback<Option<(String, String)>>,
}

#[function_component(DefendGame)]
#[rustfmt::skip]
pub fn defend_game(props: &Props) -> Html {
    let state = use_state(GameState::new);
    let interval_handle = use_mut_ref(|| None::<Interval>);
    let pressed_keys = use_mut_ref(std::collections::HashSet::<String>::new);
    let touch_controls = use_mut_ref(|| (false, false, false)); // (left, right, fire)
    let locale = use_context::<LocaleContext>().expect("locale context");

    // Keyboard and timer lifecycle manager
    {
        let pressed_keys = pressed_keys.clone();
        let interval_handle = interval_handle.clone();
        use_effect_with((), move |_| {
            let pressed_keys_down = pressed_keys.clone();
            let on_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                pressed_keys_down.borrow_mut().insert(event.key());
                if [" ", "ArrowLeft", "ArrowRight"].contains(&event.key().as_str()) {
                    event.prevent_default();
                }
            }) as Box<dyn FnMut(_)>);

            let pressed_keys_up = pressed_keys.clone();
            let on_keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                pressed_keys_up.borrow_mut().remove(&event.key());
            }) as Box<dyn FnMut(_)>);

            let window = web_sys::window().unwrap();
            window
                .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
                .unwrap();
            window
                .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
                .unwrap();

            move || {
                *interval_handle.borrow_mut() = None;
                let window = web_sys::window().unwrap();
                window
                    .remove_event_listener_with_callback(
                        "keydown",
                        on_keydown.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                window
                    .remove_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }

    let start_game = {
        let state = state.clone();
        let interval_handle = interval_handle.clone();
        let pressed_keys = pressed_keys.clone();
        let touch_controls = touch_controls.clone();
        let on_status = props.on_status.clone();
        move || {
            let mut new_state = GameState::new();
            new_state.start();
            state.set(new_state);

            let state_clone = state.clone();
            let pressed_keys = pressed_keys.clone();
            let touch_controls = touch_controls.clone();
            let on_status_tick = on_status.clone();

            let interval = Interval::new(30, move || {
                let mut current_state = (*state_clone).clone();
                if current_state.status == GameStatus::Playing {
                    // Read keys
                    let keys = pressed_keys.borrow();
                    if keys.contains("ArrowLeft") || keys.contains("a") || keys.contains("A") {
                        current_state.move_player(-3.0);
                    }
                    if keys.contains("ArrowRight") || keys.contains("d") || keys.contains("D") {
                        current_state.move_player(3.0);
                    }

                    // Read touch controls
                    let (t_left, t_right, t_fire) = *touch_controls.borrow();
                    if t_left {
                        current_state.move_player(-3.0);
                    }
                    if t_right {
                        current_state.move_player(3.0);
                    }

                    // Charge shot input resolution (Space or Touch Fire)
                    let charging_pressed = keys.contains(" ") || t_fire;
                    if charging_pressed {
                        current_state.start_charging();
                    } else if current_state.is_charging {
                        current_state.release_charge();
                    }

                    let old_status = current_state.status;
                    current_state.update();

                    if current_state.status == GameStatus::Lost && old_status == GameStatus::Playing
                    {
                        on_status_tick.emit(Some((
                            "Shields collapsed! Sector defense compromised.".to_string(),
                            "error".to_string(),
                        )));
                    }

                    state_clone.set(current_state);
                }
            });
            *interval_handle.borrow_mut() = Some(interval);

            on_status.emit(Some((
                "Defending sector against orbital debris!".to_string(),
                "success".to_string(),
            )));
        }
    };

    let stop_game = {
        let interval_handle = interval_handle.clone();
        move || {
            *interval_handle.borrow_mut() = None;
        }
    };

    let reset_game = {
        let state = state.clone();
        let stop_game = stop_game.clone();
        let on_status = props.on_status.clone();
        Callback::from(move |_| {
            stop_game();
            state.set(GameState::new());
            on_status.emit(Some((
                "Sector scanner ready for defense deployment.".to_string(),
                "success".to_string(),
            )));
        })
    };

    let on_board_click = {
        let state = state.clone();
        let start_game = start_game.clone();
        Callback::from(move |_| {
            if state.status == GameStatus::NotStarted {
                start_game();
            }
        })
    };

    #[rustfmt::skip]
    let set_touch_left = { let tc = touch_controls.clone(); move |act: bool| tc.borrow_mut().0 = act };
    #[rustfmt::skip]
    let set_touch_right = { let tc = touch_controls.clone(); move |act: bool| tc.borrow_mut().1 = act };
    #[rustfmt::skip]
    let set_touch_fire = { let tc = touch_controls.clone(); move |act: bool| tc.borrow_mut().2 = act };

    html! {
        <div class="game-container">
            <div class="board-frame" onclick={on_board_click}>
                <DefendBoard state={(*state).clone()} />

                if state.status == GameStatus::NotStarted {
                    <div class="game-overlay start-prompt glassmorphic">
                        <h2 class="outcome-title glow-cyan">{ "SECTOR THREAT DETECTED" }</h2>
                        <p class="stat-line">{ "Click grid or press restart to initialize shields." }</p>
                    </div>
                }

                if state.status == GameStatus::Lost {
                    <DefendOverlay
                        status={state.status}
                        score={state.score}
                        wave={state.wave}
                        on_restart={reset_game.clone()}
                    />
                }
            </div>

            // Controls and Stats Counter Row
            <div class="control-row-minimal">
                <div class="mode-toggles-container">
                    <DefendControls
                        on_left={Callback::from(set_touch_left.clone())}
                        on_right={Callback::from(set_touch_right.clone())}
                        on_fire={Callback::from(set_touch_fire.clone())}
                        is_charging={state.is_charging}
                        charge_level={state.charge_level}
                    />

                    if state.status == GameStatus::Playing {
                        <button onclick={let reset = reset_game.clone(); Callback::from(move |_| reset.emit(()))} class="btn-reset">
                            { locale.t("restart") }
                        </button>
                    } else if state.status == GameStatus::NotStarted {
                        <button class="btn-reset-guide" onclick={let start = start_game.clone(); Callback::from(move |_| start())}>
                            { locale.t("click_grid_to_start") }
                        </button>
                    }
                </div>

                <div class="stats-counter">
                    <div class="flags-counter">
                        <span class="hud-label">{ "SHIP:" }</span>
                        <span class="hud-value font-neon">{ format!("{}%", state.player_shield) }</span>
                    </div>
                    <div class="flags-counter">
                        <span class="hud-label">{ "PLANET:" }</span>
                        <span class="hud-value font-neon">{ format!("{}%", state.planet_shield) }</span>
                    </div>
                    <div class="flags-counter">
                        <span class="hud-label">{ "WAVE:" }</span>
                        <span class="hud-value font-neon">{ state.wave }</span>
                    </div>
                    <div class="timer-counter">
                        <span class="hud-label">{ "SCORE:" }</span>
                        <span class="hud-value font-neon">{ state.score }</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
