//! SVG canvas rendering board for Defend.
//!
//! # Coordinate System
//! The gameplay board maps to a normalized 100x100 SVG coordinate grid:
//! - X-axis: `0.0` is the left edge, `100.0` is the right edge.
//! - Y-axis: `0.0` is the top (enemy spawn area), `100.0` is the bottom (planet shield boundary).
//! - Player: Fixed at Y-coordinate `92.0`, moves horizontally along `player_x` (clamped between 6.0 and 94.0).
//! - Drone Helpers: Offset horizontally from the player ship at `player_x - 5.5` and `player_x + 5.5`.
//! - Boss: Floats horizontally at the top, centered on `boss_x` at Y-coordinate `15.0`.
//! - Projectiles: Travel upwards (decreasing Y) or downwards (increasing Y).

use crate::components::defend_logic::{GameState, GameStatus, ThreatType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DefendBoardProps {
    pub state: GameState,
}

#[function_component(DefendBoard)]
pub fn defend_board(props: &DefendBoardProps) -> Html {
    let state = &props.state;

    // The player ship is drawn as a polygon relative to the current player X coordinate (px) at Y=92.0.
    // The points trace a futuristic fighter hull with left/right wings and rear engine exhaust notches:
    // 1. Nose tip: (px, 89)
    // 2. Left cockpit edge: (px - 0.7, 90.5)
    // 3. Left wing start: (px - 0.7, 92)
    // 4. Left wing tip front: (px - 3.0, 93.5)
    // 5. Left wing tip back: (px - 3.0, 95)
    // 6. Left engine hull outer: (px - 0.7, 93.5)
    // 7. Left engine exhaust: (px - 0.7, 95)
    // 8. Center rear exhaust slot: (px, 93.5)
    // 9. Right engine exhaust: (px + 0.7, 95)
    // 10. Right engine hull outer: (px + 0.7, 93.5)
    // 11. Right wing tip back: (px + 3.0, 95)
    // 12. Right wing tip front: (px + 3.0, 93.5)
    // 13. Right wing start: (px + 0.7, 92)
    // 14. Right cockpit edge: (px + 0.7, 90.5)
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

    // Helper drones are represented by smaller triangles offset on either side of the ship.
    // Left drone is centered horizontally at `px - 5.5`, right drone at `px + 5.5`.
    let drone_points = if state.helper_time > 0 {
        let px = state.player_x;
        Some((
            format!("{},91.5 {},93.5 {},93.5", px - 5.5, px - 6.7, px - 4.3),
            format!("{},91.5 {},93.5 {},93.5", px + 5.5, px + 4.3, px + 6.7),
        ))
    } else {
        None
    };

    // Falling power-ups are rendered as diamond-shaped polygons centered around (px, py)
    // with a horizontal and vertical radius of 2.5.
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

    // The charging orb is a glowing circle drawn right at the weapon mount (Y = 88.0).
    // The radius increases proportionally to the charge level, capped visually.
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

    // The boss ship is rendered at the top as a large polygon centered on `bx` around Y=15.
    // The geometry maps a heavy cruiser hull:
    // 1. Nose / Front Cannon: (bx, 8)
    // 2. Left Shoulder: (bx - 7.0, 12)
    // 3. Left Wingtip: (bx - 9.0, 18)
    // 4. Left Thruster: (bx - 4.0, 18)
    // 5. Rear Center Exhaust: (bx, 14)
    // 6. Right Thruster: (bx + 4.0, 18)
    // 7. Right Wingtip: (bx + 9.0, 18)
    // 8. Right Shoulder: (bx + 7.0, 12)
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

    // Computes the red filled portion of the boss health bar (width scaled up to 50 SVG units).
    let boss_health_bar_width = state
        .boss_health
        .map(|bh| format!("{}", (bh as f64 / state.boss_max_health as f64) * 50.0));

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
                        match threat.kind {
                            ThreatType::Bullet => {
                                html! { <line x1={threat.x.to_string()} y1={threat.y.to_string()} x2={threat.x.to_string()} y2={(threat.y + 2.5).to_string()} class="neon-enemy-bullet" /> }
                            }
                            ThreatType::Scout => {
                                let (tx, ty, s) = (threat.x, threat.y, threat.size);
                                let points = format!("{},{} {},{} {},{} {},{} {},{}", tx, ty - s, tx + s, ty + s*0.3, tx + s*0.3, ty + s, tx - s*0.3, ty + s, tx - s, ty + s*0.3);
                                html! { <polygon points={points} class="neon-scout" /> }
                            }
                            ThreatType::Asteroid => {
                                let (tx, ty, s) = (threat.x, threat.y, threat.size);
                                let points = format!("{},{} {},{} {},{} {},{}", tx, ty - s, tx + s, ty, tx, ty + s, tx - s, ty);
                                html! { <polygon points={points} class="neon-threat" /> }
                            }
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
