//! Phases 1-6: stars, helper weapons, beam power shot, power-ups,
//! weapon charging, and boss update. See `tick/mod.rs` for the
//! orchestrator and the phase numbering.

use super::super::types::*;

impl GameState {
    /// Phase 1: scroll background stars downward; wrap to top with a
    /// new horizontal position.
    pub(super) fn advance_stars(&mut self) {
        for star in &mut self.stars {
            star.y += star.speed;
            if star.y > 100.0 {
                star.y = 0.0;
                star.x = js_sys::Math::random() * 100.0;
            }
        }
    }

    /// Phase 2: while the helper-drone timer is active, fire twin
    /// tracking lasers at the closest threat every 25 ticks.
    pub(super) fn update_helper_weapons(&mut self) {
        if self.helper_time == 0 {
            return;
        }
        self.helper_time -= 1;
        if !self.ticks.is_multiple_of(25) || self.threats.is_empty() {
            return;
        }
        let (mut target_idx, mut min_dy) = (0, f64::MAX);
        for (i, t) in self.threats.iter().enumerate() {
            let dy = 92.0 - t.y;
            if dy > 0.0 && dy < min_dy {
                min_dy = dy;
                target_idx = i;
            }
        }
        let t = &self.threats[target_idx];
        for lx in [self.player_x - 5.5, self.player_x + 5.5] {
            let (dx, dy) = (t.x - lx, t.y - 92.0);
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            self.lasers.push(Laser {
                x: lx,
                y: 92.0,
                vx: (dx / len) * 2.2,
                vy: (dy / len) * 2.2,
                is_charge_shot: false,
                radius: 0.8,
            });
        }
    }

    /// Phase 3: while the beam timer is active, sweep a vertical beam
    /// that destroys any threat in its lane, and emit particles.
    pub(super) fn update_beam_power_shot(&mut self) {
        if self.beam_time == 0 {
            return;
        }
        self.beam_time -= 1;
        let bx = self.player_x;
        let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
        let old = std::mem::take(&mut self.threats);
        let mut rem = Vec::new();
        for t in old {
            if (t.x - bx).abs() < limit && t.y < 88.0 {
                self.score += 15;
                self.spawn_explosion(t.x, t.y, 8);
            } else {
                rem.push(t);
            }
        }
        self.threats = rem;
        if self.ticks.is_multiple_of(2) {
            let py = js_sys::Math::random() * 80.0;
            self.particles.push(Particle {
                x: bx + js_sys::Math::random() * 2.0 - 1.0,
                y: py,
                vx: js_sys::Math::random() * 0.8 - 0.4,
                vy: js_sys::Math::random() * 0.8 - 0.4,
                life: 0.5,
            });
        }
    }

    /// Phase 4: drift a power-up toward the player; collect it on
    /// overlap. Spawn a new power-up every 300 ticks.
    pub(super) fn update_powerups(&mut self) {
        if self.powerup_type > 0 {
            self.powerup_y += 0.55;
            if self.powerup_y > 100.0 {
                self.powerup_type = 0;
            } else if self.powerup_y >= 90.0
                && self.powerup_y <= 95.0
                && (self.powerup_x - self.player_x).abs() < 5.0
            {
                if self.powerup_type == 1 {
                    self.player_shield = (self.player_shield + 25).min(100);
                } else {
                    self.helper_time = 450;
                }
                self.spawn_explosion(self.powerup_x, self.powerup_y, 10);
                self.powerup_type = 0;
            }
        } else if self.ticks.is_multiple_of(300) {
            self.powerup_x = js_sys::Math::random() * 80.0 + 10.0;
            self.powerup_y = 0.0;
            self.powerup_type = if js_sys::Math::random() > 0.5 { 1 } else { 2 };
        }
    }

    /// Phase 5: while charging, accumulate charge level and emit
    /// spark particles.
    pub(super) fn update_charging(&mut self) {
        if !self.is_charging {
            return;
        }
        self.charge_level = (self.charge_level + 0.025).min(2.0);
        if self.ticks.is_multiple_of(2) {
            let angle = js_sys::Math::random() * std::f64::consts::TAU;
            let dist = js_sys::Math::random() * 8.0 + 4.0;
            let px = self.player_x + angle.cos() * dist;
            let py = 87.0 + angle.sin() * dist;
            self.particles.push(Particle {
                x: px,
                y: py,
                vx: (self.player_x - px) * 0.12,
                vy: (87.0 - py) * 0.12,
                life: 0.8,
            });
        }
    }

    /// Phase 6: boss movement, bullet fire, laser hit detection,
    /// beam damage, and boss defeat handling.
    pub(super) fn update_boss(&mut self) {
        let Some(mut bh) = self.boss_health else {
            return;
        };
        self.boss_x += self.boss_vx;
        if self.boss_x > 90.0 || self.boss_x < 10.0 {
            self.boss_vx = -self.boss_vx;
        }
        if self.ticks.is_multiple_of(40) {
            self.threats.push(Threat {
                x: self.boss_x,
                y: 17.0,
                speed: 1.2,
                size: 1.0,
                kind: ThreatType::Bullet,
            });
        }
        let mut hit_lasers = std::collections::HashSet::new();
        let mut explosions = Vec::new();
        for (idx, l) in self.lasers.iter().enumerate() {
            if (l.x - self.boss_x).abs() < 8.0 && (l.y - 15.0).abs() < 5.0 {
                bh = bh.saturating_sub(if l.is_charge_shot { 50 } else { 10 });
                explosions.push((l.x, l.y));
                hit_lasers.insert(idx);
            }
        }
        for (ex, ey) in explosions {
            self.spawn_explosion(ex, ey, 6);
        }
        if self.beam_time > 0 {
            let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
            if (self.boss_x - self.player_x).abs() < limit {
                bh = bh.saturating_sub(2);
            }
        }
        if bh == 0 {
            self.boss_health = None;
            self.score += 500;
            self.spawn_explosion(self.boss_x, 15.0, 40);
            self.wave += 1;
        } else {
            self.boss_health = Some(bh);
        }
        let old_lasers = std::mem::take(&mut self.lasers);
        self.lasers = old_lasers
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !hit_lasers.contains(i))
            .map(|(_, l)| l)
            .collect();
    }
}
