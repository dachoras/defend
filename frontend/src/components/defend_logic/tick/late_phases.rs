//! Phases 7-13: spawning, wave progression, projectile/threat
//! motion, threat collisions, laser collisions, particles. See
//! `tick/mod.rs` for the orchestrator and the phase numbering.

use super::super::types::*;

impl GameState {
    /// Phase 7: standard threat spawning (scouts + asteroids), plus
    /// scout-fired bullets every 75 ticks.
    pub(super) fn update_spawning(&mut self) {
        if self.boss_health.is_some() {
            return;
        }
        let spawn_interval = (35 - (self.wave as i32 * 2)).max(10) as u64;
        if self.ticks.is_multiple_of(spawn_interval) {
            let (x, is_s) = (
                js_sys::Math::random() * 90.0 + 5.0,
                js_sys::Math::random() > 0.5,
            );
            let (spd, sz, k) = if is_s {
                (
                    js_sys::Math::random() * 0.35 + 0.45 + (self.wave as f64 * 0.05),
                    2.2,
                    ThreatType::Scout,
                )
            } else {
                (
                    js_sys::Math::random() * 0.25 + 0.25 + (self.wave as f64 * 0.04),
                    3.0,
                    ThreatType::Asteroid,
                )
            };
            self.threats.push(Threat {
                x,
                y: 0.0,
                speed: spd,
                size: sz,
                kind: k,
            });
        }
        if self.ticks.is_multiple_of(75)
            && let Some(t) = self
                .threats
                .iter()
                .find(|t| t.kind == ThreatType::Scout && t.y > 10.0 && t.y < 70.0)
        {
            let bx = t.x;
            let by = t.y + 2.0;
            let bspd = t.speed + 0.6;
            self.threats.push(Threat {
                x: bx,
                y: by,
                speed: bspd,
                size: 0.9,
                kind: ThreatType::Bullet,
            });
        }
    }

    /// Phase 8: every 600 ticks, advance the wave counter; every
    /// 10th wave spawns a new boss.
    pub(super) fn update_wave_progression(&mut self) {
        if !self.ticks.is_multiple_of(600) || self.boss_health.is_some() {
            return;
        }
        self.wave += 1;
        if self.wave.is_multiple_of(10) {
            let hp = 150 + self.wave * 10;
            self.boss_health = Some(hp);
            self.boss_max_health = hp;
            self.boss_x = 50.0;
            self.boss_vx = 0.8;
            self.threats.clear();
        }
    }

    /// Phase 9: move lasers; cull off-screen ones.
    pub(super) fn move_projectiles(&mut self) {
        for laser in &mut self.lasers {
            laser.x += laser.vx;
            laser.y += laser.vy;
        }
        self.lasers
            .retain(|l| l.y > 0.0 && l.y < 100.0 && l.x > 0.0 && l.x < 100.0);
    }

    /// Phase 10: move threats down by their per-frame speed.
    pub(super) fn move_threats(&mut self) {
        for threat in &mut self.threats {
            threat.y += threat.speed;
        }
    }

    /// Phase 11: threats that overlap the player damage the shield;
    /// asteroids past the bottom damage the planet; scouts wrap
    /// around to the top. Sets `GameStatus::Lost` when a shield hits 0.
    pub(super) fn handle_threat_collisions(&mut self) {
        let old_threats = std::mem::take(&mut self.threats);
        let mut new_threats = Vec::new();
        for mut threat in old_threats {
            if threat.y >= 90.0 && threat.y <= 95.0 && (threat.x - self.player_x).abs() < 5.0 {
                let dmg = if threat.kind == ThreatType::Bullet {
                    15
                } else {
                    20
                };
                self.player_shield = self.player_shield.saturating_sub(dmg);
                self.spawn_explosion(threat.x, threat.y, 8);
                if self.player_shield == 0 {
                    self.status = GameStatus::Lost;
                }
            } else if threat.y >= 100.0 {
                if threat.kind == ThreatType::Scout {
                    threat.y = 0.0;
                    threat.x = js_sys::Math::random() * 90.0 + 5.0;
                    new_threats.push(threat);
                } else if threat.kind == ThreatType::Asteroid {
                    self.planet_shield = self.planet_shield.saturating_sub(10);
                    if self.planet_shield == 0 {
                        self.status = GameStatus::Lost;
                    }
                }
            } else {
                new_threats.push(threat);
            }
        }
        self.threats = new_threats;
    }

    /// Phase 13: drift particles outward and shrink their life; cull
    /// dead particles.
    pub(super) fn update_particles(&mut self) {
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.04;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}
