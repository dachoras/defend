use super::types::*;

impl GameState {
    pub fn update(&mut self) {
        self.ticks += 1;

        // 1. Move stars for parallax scroll
        for star in &mut self.stars {
            star.y += star.speed;
            if star.y > 100.0 {
                star.y = 0.0;
                star.x = js_sys::Math::random() * 100.0;
            }
        }

        // 2. Drone Helper Weapons: Auto-fire helper lasers at closest targets
        if self.helper_time > 0 {
            self.helper_time -= 1;
            if self.ticks % 25 == 0 && !self.threats.is_empty() {
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
        }

        // 3. Vertical Beam Power Shot
        if self.beam_time > 0 {
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
            if self.ticks % 2 == 0 {
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

        // 4. Power-Up Spawning and Collection
        if self.powerup_type > 0 {
            self.powerup_y += 0.55;
            if self.powerup_y > 100.0 {
                self.powerup_type = 0;
            } else if self.powerup_y >= 90.0 && self.powerup_y <= 95.0 && (self.powerup_x - self.player_x).abs() < 5.0 {
                if self.powerup_type == 1 {
                    self.player_shield = (self.player_shield + 25).min(100);
                } else {
                    self.helper_time = 450;
                }
                self.spawn_explosion(self.powerup_x, self.powerup_y, 10);
                self.powerup_type = 0;
            }
        } else if self.ticks % 300 == 0 {
            self.powerup_x = js_sys::Math::random() * 80.0 + 10.0;
            self.powerup_y = 0.0;
            self.powerup_type = if js_sys::Math::random() > 0.5 { 1 } else { 2 };
        }

        // 5. Weapon Charging
        if self.is_charging {
            self.charge_level = (self.charge_level + 0.025).min(2.0);
            if self.ticks % 2 == 0 {
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

        // 6. Boss Update
        if let Some(mut bh) = self.boss_health {
            self.boss_x += self.boss_vx;
            if self.boss_x > 90.0 || self.boss_x < 10.0 {
                self.boss_vx = -self.boss_vx;
            }
            if self.ticks % 40 == 0 {
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

        // 7. Standard Spawning
        if self.boss_health.is_none() {
            let spawn_interval = (35 - (self.wave as i32 * 2)).max(10) as u64;
            if self.ticks.is_multiple_of(spawn_interval) {
                let (x, is_s) = (js_sys::Math::random() * 90.0 + 5.0, js_sys::Math::random() > 0.5);
                let (spd, sz, k) = if is_s {
                    (js_sys::Math::random() * 0.35 + 0.45 + (self.wave as f64 * 0.05), 2.2, ThreatType::Scout)
                } else {
                    (js_sys::Math::random() * 0.25 + 0.25 + (self.wave as f64 * 0.04), 3.0, ThreatType::Asteroid)
                };
                self.threats.push(Threat { x, y: 0.0, speed: spd, size: sz, kind: k });
            }
            if self.ticks % 75 == 0 {
                if let Some(t) = self.threats.iter().find(|t| t.kind == ThreatType::Scout && t.y > 10.0 && t.y < 70.0) {
                    let bx = t.x;
                    let by = t.y + 2.0;
                    let bspd = t.speed + 0.6;
                    self.threats.push(Threat { x: bx, y: by, speed: bspd, size: 0.9, kind: ThreatType::Bullet });
                }
            }
        }

        // 8. Wave Progression / Boss Spawning
        if self.ticks.is_multiple_of(600) && self.boss_health.is_none() {
            self.wave += 1;
            if self.wave % 10 == 0 {
                let hp = 150 + self.wave * 10;
                self.boss_health = Some(hp);
                self.boss_max_health = hp;
                self.boss_x = 50.0;
                self.boss_vx = 0.8;
                self.threats.clear();
            }
        }

        // 9. Move Projectiles
        for laser in &mut self.lasers {
            laser.x += laser.vx;
            laser.y += laser.vy;
        }
        self.lasers.retain(|l| l.y > 0.0 && l.y < 100.0 && l.x > 0.0 && l.x < 100.0);

        // 10. Move threats
        for threat in &mut self.threats {
            threat.y += threat.speed;
        }

        // 11. Threat Collisions with Player or Bottom
        let old_threats = std::mem::take(&mut self.threats);
        let mut new_threats = Vec::new();
        for mut threat in old_threats {
            if threat.y >= 90.0 && threat.y <= 95.0 && (threat.x - self.player_x).abs() < 5.0 {
                let dmg = if threat.kind == ThreatType::Bullet { 15 } else { 20 };
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

        // 12. Laser-Threat Collision Detection
        self.handle_collisions();

        // 13. Move and fade particle effects
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.04;
        }
        self.particles.retain(|p| p.life > 0.0);
    }
}
