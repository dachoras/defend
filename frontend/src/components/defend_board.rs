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
            "{},89 {},90.5 {},92 {},93.5 {},95 {},93.5 {},95 {},93.5 {},95 {},93.5 {},95 {},93.5 {},92 {},90.5",
            px,
            px - 0.7,
            px - 0.7,
            px - 3.0,
            px - 3.0,
            px - 0.7,
            px - 0.7,
            px,
            px + 0.7,
            px + 0.7,
            px + 3.0,
            px + 3.0,
            px + 0.7,
            px + 0.7
        ))
    } else {
        None
    };

    let drone_points = if state.helper_time > 0 {
        let px = state.player_x;
        Some((
            format!("{},91.5 {},93.5 {},93.5", px - 5.5, px - 6.7, px - 4.3),
            format!("{},91.5 {},93.5 {},93.5", px + 5.5, px + 4.3, px + 6.7),
        ))
    } else {
        None
    };

    let powerup_points = if state.powerup_type > 0 {
        let (px, py) = (state.powerup_x, state.powerup_y);
        let pts = format!(
            "{},{} {},{} {},{} {},{}",
            px,
            py - 2.5,
            px + 2.5,
            py,
            px,
            py + 2.5,
            px - 2.5,
            py
        );
        let class = if state.powerup_type == 1 {
            "neon-shield-powerup"
        } else {
            "neon-helper-powerup"
        };
        Some((pts, class))
    } else {
        None
    };

    let charge_orb = if state.is_charging {
        let r = (state.charge_level * 3.5).max(0.5);
        let orb_class = if state.charge_level >= 2.0 {
            "neon-charge-orb level2-charged"
        } else if state.charge_level >= 1.0 {
            "neon-charge-orb fully-charged"
        } else {
            "neon-charge-orb"
        };
        Some((r, orb_class))
    } else {
        None
    };

    let boss_points = if state.boss_health.is_some() {
        let bx = state.boss_x;
        Some(format!(
            "{},8 {},12 {},18 {},18 {},14 {},18 {},18 {},12",
            bx,
            bx - 7.0,
            bx - 9.0,
            bx - 4.0,
            bx,
            bx + 4.0,
            bx + 9.0,
            bx + 7.0
        ))
    } else {
        None
    };

    let boss_health_bar_width = if let Some(bh) = state.boss_health {
        Some(format!(
            "{}",
            (bh as f64 / state.boss_max_health as f64) * 50.0
        ))
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

                {
                    for state.stars.iter().map(|star| {
                        let opacity = 0.15 + (star.speed * 0.5);
                        html! { <circle cx={star.x.to_string()} cy={star.y.to_string()} r={star.size.to_string()} style={format!("opacity: {}; fill: #ffffff;", opacity)} /> }
                    })
                }

                if let Some(points) = ship_points {
                    <polygon points={points} class="neon-player-ship" />
                }

                if let Some((left, right)) = drone_points {
                    <polygon points={left} class="neon-helper-drone" />
                    <polygon points={right} class="neon-helper-drone" />
                }

                if let Some((points, class)) = powerup_points {
                    <polygon points={points} class={class} />
                }

                if let Some((r, orb_class)) = charge_orb {
                    <circle cx={state.player_x.to_string()} cy="88" r={r.to_string()} class={orb_class} />
                }

                if state.beam_time > 0 {
                    <line x1={state.player_x.to_string()} y1="88" x2={state.player_x.to_string()} y2="0" class="neon-level2-beam" />
                    <line x1={state.player_x.to_string()} y1="88" x2={state.player_x.to_string()} y2="0" class="neon-level2-beam-core" />
                    if state.helper_time > 0 {
                        <line x1={(state.player_x - 5.5).to_string()} y1="92" x2={(state.player_x - 5.5).to_string()} y2="0" class="neon-level2-beam" />
                        <line x1={(state.player_x - 5.5).to_string()} y1="92" x2={(state.player_x - 5.5).to_string()} y2="0" class="neon-level2-beam-core" />
                        <line x1={(state.player_x + 5.5).to_string()} y1="92" x2={(state.player_x + 5.5).to_string()} y2="0" class="neon-level2-beam" />
                        <line x1={(state.player_x + 5.5).to_string()} y1="92" x2={(state.player_x + 5.5).to_string()} y2="0" class="neon-level2-beam-core" />
                    }
                }

                if let Some(points) = boss_points {
                    <polygon points={points} class="neon-boss-ship" />
                }

                if let Some(w) = boss_health_bar_width {
                    <rect x="25" y="4" width="50" height="1.2" fill="#333333" rx="0.3" />
                    <rect x="25" y="4" width={w} height="1.2" fill="#ef4444" rx="0.3" class="neon-boss-health-bar" />
                    <text x="50" y="3" fill="#ef4444" font-size="2" font-family="monospace" text-anchor="middle" class="neon-boss-label">{ "BOSS THREAT" }</text>
                }

                {
                    for state.lasers.iter().map(|laser| {
                        if laser.is_charge_shot {
                            html! { <circle cx={laser.x.to_string()} cy={laser.y.to_string()} r={laser.radius.to_string()} class="neon-charge-shot" /> }
                        } else {
                            html! { <line x1={laser.x.to_string()} y1={laser.y.to_string()} x2={(laser.x - laser.vx * 1.5).to_string()} y2={(laser.y - laser.vy * 1.5).to_string()} class="neon-laser" /> }
                        }
                    })
                }

                {
                    for state.threats.iter().map(|threat| {
                        if threat.is_bullet {
                            html! { <line x1={threat.x.to_string()} y1={threat.y.to_string()} x2={threat.x.to_string()} y2={(threat.y + 2.5).to_string()} class="neon-enemy-bullet" /> }
                        } else {
                            let (tx, ty, s) = (threat.x, threat.y, threat.size);
                            html! { <polygon points={format!("{},{} {},{} {},{} {},{}", tx, ty - s, tx + s, ty, tx, ty + s, tx - s, ty)} class="neon-threat" /> }
                        }
                    })
                }

                {
                    for state.particles.iter().map(|p| {
                        html! { <circle cx={p.x.to_string()} cy={p.y.to_string()} r={(0.6 * p.life).to_string()} style={format!("opacity: {}", p.life)} class="neon-particle" /> }
                    })
                }
            </svg>
        </div>
    }
}
