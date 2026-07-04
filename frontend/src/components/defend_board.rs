use crate::components::defend_logic::{GameState, GameStatus};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DefendBoardProps {
    pub state: GameState,
}

#[function_component(DefendBoard)]
pub fn defend_board(props: &DefendBoardProps) -> Html {
    let state = &props.state;

    let ship_points = if state.status == GameStatus::Playing {
        let px = state.player_x;
        Some(format!(
            "{},87 {},89 {},91 {},93 {},95 {},93 {},95 {},93 {},95 {},93 {},95 {},93 {},91 {},89",
            px,
            px - 1.0,
            px - 1.0,
            px - 4.0,
            px - 4.0,
            px - 1.0,
            px - 1.0,
            px,
            px + 1.0,
            px + 1.0,
            px + 4.0,
            px + 4.0,
            px + 1.0,
            px + 1.0
        ))
    } else {
        None
    };

    let charge_orb = if state.is_charging {
        let r = (state.charge_level * 3.5).max(0.5);
        let orb_class = if state.charge_level >= 1.0 {
            "neon-charge-orb fully-charged"
        } else {
            "neon-charge-orb"
        };
        Some((r, orb_class))
    } else {
        None
    };

    html! {
        <div class="defend-board-container">
            <svg class="defend-svg-canvas" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet">
                <defs>
                    <pattern id="grid-pattern" width="10" height="10" patternUnits="userSpaceOnUse">
                        <path d="M 10 0 L 0 0 0 10" fill="none" stroke="rgba(255, 255, 255, 0.03)" stroke-width="0.5" />
                    </pattern>
                </defs>
                <rect width="100%" height="100%" fill="url(#grid-pattern)" />

                // Render background stars (parallax dust)
                {
                    for state.stars.iter().map(|star| {
                        let opacity = 0.15 + (star.speed * 0.5);
                        let style = format!("opacity: {}; fill: #ffffff;", opacity);
                        html! {
                            <circle
                                cx={star.x.to_string()}
                                cy={star.y.to_string()}
                                r={star.size.to_string()}
                                style={style}
                            />
                        }
                    })
                }

                if let Some(points) = ship_points {
                    <polygon
                        points={points}
                        class="neon-player-ship"
                    />
                }

                if let Some((r, orb_class)) = charge_orb {
                    <circle
                        cx={state.player_x.to_string()}
                        cy="86"
                        r={r.to_string()}
                        class={orb_class}
                    />
                }

                // Render lasers (cyan neon pulses or massive charge blasts)
                {
                    for state.lasers.iter().map(|laser| {
                        if laser.is_charge_shot {
                            html! {
                                <circle
                                    cx={laser.x.to_string()}
                                    cy={laser.y.to_string()}
                                    r={laser.radius.to_string()}
                                    class="neon-charge-shot"
                                />
                            }
                        } else {
                            html! {
                                <line
                                    x1={laser.x.to_string()}
                                    y1={laser.y.to_string()}
                                    x2={laser.x.to_string()}
                                    y2={(laser.y - 3.0).to_string()}
                                    class="neon-laser"
                                />
                            }
                        }
                    })
                }

                // Render threats (neon red/orange polygons/diamonds)
                {
                    for state.threats.iter().map(|threat| {
                        let tx = threat.x;
                        let ty = threat.y;
                        let s = threat.size;
                        let points = format!("{},{} {},{} {},{} {},{}", tx, ty - s, tx + s, ty, tx, ty + s, tx - s, ty);
                        html! {
                            <polygon
                                points={points}
                                class="neon-threat"
                            />
                        }
                    })
                }

                // Render explosion particles (yellow/orange sparks fading out)
                {
                    for state.particles.iter().map(|p| {
                        let opacity = p.life;
                        let radius = 0.6 * p.life;
                        let style = format!("opacity: {}", opacity);
                        html! {
                            <circle
                                cx={p.x.to_string()}
                                cy={p.y.to_string()}
                                r={radius.to_string()}
                                style={style}
                                class="neon-particle"
                            />
                        }
                    })
                }
            </svg>
        </div>
    }
}
