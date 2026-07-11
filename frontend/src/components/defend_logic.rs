//! Defend game logic and grid state management.
//!
//! Handles movement, threat spawning, collisions, and state updates
//! for players, lasers, power-ups, particles, and bosses.

/// Status of the current game session.
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum GameStatus {
    /// The game is waiting to be started.
    NotStarted,
    /// The gameplay loop is actively running.
    Playing,
    /// The player has lost (either due to player shield or planet shield depletion).
    Lost,
}

/// Category of threat spawned from the top of the grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ThreatType {
    /// Heavy rocks that cause planet shield damage if they bypass the player.
    Asteroid,
    /// Fast moving scout ships that shoot back.
    Scout,
    /// Enemy projectiles fired by scout ships or the boss.
    Bullet,
}

/// A laser projectile fired by the player.
#[derive(Clone, Debug, PartialEq)]
pub struct Laser {
    /// Horizontal position on the 0-100 grid.
    pub x: f64,
    /// Vertical position on the 0-100 grid.
    pub y: f64,
    /// Horizontal velocity.
    pub vx: f64,
    /// Vertical velocity.
    pub vy: f64,
    /// Whether this is a high-damage charged shot that pierces enemies.
    pub is_charge_shot: bool,
    /// Hitbox radius of the laser.
    pub radius: f64,
}

/// An active threat moving down the grid.
#[derive(Clone, Debug, PartialEq)]
pub struct Threat {
    /// Horizontal position on the 0-100 grid.
    pub x: f64,
    /// Vertical position on the 0-100 grid.
    pub y: f64,
    /// Falling speed per tick.
    pub speed: f64,
    /// Collision radius / size.
    pub size: f64,
    /// The type of threat.
    pub kind: ThreatType,
}

/// Visual particle effect spawned upon explosions or charging.
#[derive(Clone, Debug, PartialEq)]
pub struct Particle {
    /// Horizontal position.
    pub x: f64,
    /// Vertical position.
    pub y: f64,
    /// Horizontal velocity.
    pub vx: f64,
    /// Vertical velocity.
    pub vy: f64,
    /// Opacity/remaining lifespan in range 0.0..=1.0.
    pub life: f64,
}

/// Background stars moving downwards to create a parallax scrolling effect.
#[derive(Clone, Debug, PartialEq)]
pub struct Star {
    /// Horizontal position.
    pub x: f64,
    /// Vertical position.
    pub y: f64,
    /// Scrolling speed.
    pub speed: f64,
    /// Drawing size.
    pub size: f64,
}

/// Central state manager for the Defend gameplay loop.
#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    /// Player's horizontal position. Player is fixed at Y = 92.0.
    pub player_x: f64,
    /// Active lasers currently in flight.
    pub lasers: Vec<Laser>,
    /// Active threat entities moving down the grid.
    pub threats: Vec<Threat>,
    /// Active cosmetic explosion and charge particles.
    pub particles: Vec<Particle>,
    /// The current score.
    pub score: u32,
    /// Player's shield percentage (0 to 100).
    pub player_shield: u32,
    /// The planet's shield percentage (0 to 100).
    pub planet_shield: u32,
    /// The current wave level.
    pub wave: u32,
    /// Status of the game.
    pub status: GameStatus,
    /// Total ticks elapsed since the game started.
    pub ticks: u64,
    /// Power shot charge level (0.0 up to 2.0).
    pub charge_level: f64,
    /// Whether the player is currently holding down the fire/charge key.
    pub is_charging: bool,
    /// Parallax star field.
    pub stars: Vec<Star>,
    /// X coordinate of falling power-ups.
    pub powerup_x: f64,
    /// Y coordinate of falling power-ups.
    pub powerup_y: f64,
    /// Type of power-up: 0 = none, 1 = shield repair, 2 = drones/helpers.
    pub powerup_type: u8,
    /// Remaining duration (ticks) of active drone helpers.
    pub helper_time: u32,
    /// Remaining duration (ticks) of the vertical laser beam.
    pub beam_time: u32,
    /// Current health of the wave boss, if active.
    pub boss_health: Option<u32>,
    /// Maximum health of the active wave boss.
    pub boss_max_health: u32,
    /// Boss ship horizontal position.
    pub boss_x: f64,
    /// Boss ship horizontal velocity.
    pub boss_vx: f64,
}

impl GameState {
    /// Create a new, unstarted game state with a randomized star field.
    pub fn new() -> Self {
        Self {
            player_x: 50.0,
            lasers: Vec::new(),
            threats: Vec::new(),
            particles: Vec::new(),
            score: 0,
            player_shield: 100,
            planet_shield: 100,
            wave: 1,
            status: GameStatus::NotStarted,
            ticks: 0,
            charge_level: 0.0,
            is_charging: false,
            stars: (0..22)
                .map(|_| Star {
                    x: js_sys::Math::random() * 100.0,
                    y: js_sys::Math::random() * 100.0,
                    speed: js_sys::Math::random() * 0.45 + 0.15,
                    size: js_sys::Math::random() * 0.45 + 0.15,
                })
                .collect(),
            powerup_x: 0.0,
            powerup_y: 0.0,
            powerup_type: 0,
            helper_time: 0,
            beam_time: 0,
            boss_health: None,
            boss_max_health: 100,
            boss_x: 0.0,
            boss_vx: 0.0,
        }
    }

    /// Reset and start a new gameplay session.
    pub fn start(&mut self) {
        *self = Self::new();
        self.status = GameStatus::Playing;
    }

    /// Primary game loop update step. Called on every tick (~60fps).
    pub fn update(&mut self) {
        if self.status != GameStatus::Playing {
            return;
        }
        self.ticks += 1;

        // 1. Move background stars downwards. Wrap when they scroll past the bottom (Y > 100).
        for s in &mut self.stars {
            s.y += s.speed;
            if s.y > 100.0 {
                s.y = 0.0;
                s.x = js_sys::Math::random() * 100.0;
            }
        }

        // 2. Active Helper Drones: Fire double lasers automatically at the closest target.
        if self.helper_time > 0 {
            self.helper_time -= 1;
            // Drones target threats every 18 ticks.
            if self.helper_time % 18 == 0 && !self.threats.is_empty() {
                let (mut target_idx, mut min_dy) = (0, f64::MAX);
                // Find the closest threat vertically above the player.
                for (i, t) in self.threats.iter().enumerate() {
                    let dy = 92.0 - t.y;
                    if dy > 0.0 && dy < min_dy {
                        min_dy = dy;
                        target_idx = i;
                    }
                }
                let t = &self.threats[target_idx];
                // Spawn one laser from each side drone helper.
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

        // 3. Vertical Beam Power Shot: Periodically disintegrates all threats in its vertical column.
        if self.beam_time > 0 {
            self.beam_time -= 1;
            let bx = self.player_x;
            // The beam width increases if drones are active.
            let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
            let old = std::mem::take(&mut self.threats);
            let mut rem = Vec::new();
            for t in old {
                // Destroy threats within the beam path.
                if (t.x - bx).abs() < limit && t.y < 88.0 {
                    self.score += 15;
                    self.spawn_explosion(t.x, t.y, 8);
                } else {
                    rem.push(t);
                }
            }
            self.threats = rem;
            // Add cosmetic beam energy particles.
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

        // 4. Power-Up Spawning and Collection:
        if self.powerup_type > 0 {
            // Drop powerup down the screen.
            self.powerup_y += 0.55;
            if self.powerup_y > 100.0 {
                self.powerup_type = 0; // Despawn when out of bounds.
            } else if self.powerup_y >= 90.0 && self.powerup_y <= 95.0 && (self.powerup_x - self.player_x).abs() < 5.0 {
                // Collect power-up!
                if self.powerup_type == 1 {
                    self.player_shield = (self.player_shield + 25).min(100);
                } else {
                    self.helper_time = 450;
                }
                self.spawn_explosion(self.powerup_x, self.powerup_y, 10);
                self.powerup_type = 0;
            }
        } else if self.ticks % 300 == 0 {
            // Roll to spawn a new power-up every 300 ticks.
            self.powerup_x = js_sys::Math::random() * 80.0 + 10.0;
            self.powerup_y = 0.0;
            self.powerup_type = if js_sys::Math::random() > 0.5 { 1 } else { 2 };
        }

        // 5. Weapon Charging: Accumulate charge level and pull particles inward.
        if self.is_charging {
            self.charge_level = (self.charge_level + 0.025).min(2.0);
            if self.ticks % 2 == 0 {
                // Pull surrounding particles toward player ship coordinates (Y = 87.0).
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

        // 6. Boss Update: Handles health depletion, movement bouncing, and projectile firing.
        if let Some(mut bh) = self.boss_health {
            self.boss_x += self.boss_vx;
            // Bounce horizontal velocity at grid boundaries (10.0 and 90.0).
            if self.boss_x > 90.0 || self.boss_x < 10.0 {
                self.boss_vx = -self.boss_vx;
            }
            // Fire bullet projectile.
            if self.ticks % 40 == 0 {
                self.threats.push(Threat {
                    x: self.boss_x,
                    y: 17.0,
                    speed: 1.2,
                    size: 1.0,
                    kind: ThreatType::Bullet,
                });
            }
            // Check laser collisions on boss body.
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
            // Continuous beam damage.
            if self.beam_time > 0 {
                let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
                if (self.boss_x - self.player_x).abs() < limit {
                    bh = bh.saturating_sub(2);
                }
            }
            // Handle boss death.
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

        // 7. Standard Spawning logic (active when Boss is dead/inactive):
        if self.boss_health.is_none() {
            // Speed up spawns as wave number increases.
            let spawn_interval = (35 - (self.wave as i32 * 2)).max(10) as u64;
            if self.ticks.is_multiple_of(spawn_interval) {
                let (x, is_s) = (js_sys::Math::random() * 90.0 + 5.0, js_sys::Math::random() > 0.5);
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
            // Scout ships shoot bullets periodically.
            if self.ticks % 75 == 0 {
                if let Some(t) = self.threats
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
        }

        // 8. Wave Progression / Boss Spawning:
        // Every 600 ticks, advance wave, and spawn boss every 10th wave.
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

        // 9. Move Projectiles (Lasers) and prune out-of-bounds lasers.
        for laser in &mut self.lasers {
            laser.x += laser.vx;
            laser.y += laser.vy;
        }
        self.lasers.retain(|l| l.y > 0.0 && l.y < 100.0 && l.x > 0.0 && l.x < 100.0);

        // 10. Move threats.
        for threat in &mut self.threats {
            threat.y += threat.speed;
        }

        // 11. Threat Collisions with Player or Bottom Grid Line:
        let old_threats = std::mem::take(&mut self.threats);
        let mut new_threats = Vec::new();
        for mut threat in old_threats {
            if threat.y >= 90.0 && threat.y <= 95.0 && (threat.x - self.player_x).abs() < 5.0 {
                // Collided with player! Deduct player shield.
                let dmg = if threat.kind == ThreatType::Bullet { 15 } else { 20 };
                self.player_shield = self.player_shield.saturating_sub(dmg);
                self.spawn_explosion(threat.x, threat.y, 8);
                if self.player_shield == 0 {
                    self.status = GameStatus::Lost;
                }
            } else if threat.y >= 100.0 {
                // Reached the bottom edge.
                if threat.kind == ThreatType::Scout {
                    // Scouts wrap around to fly down again.
                    threat.y = 0.0;
                    threat.x = js_sys::Math::random() * 90.0 + 5.0;
                    new_threats.push(threat);
                } else if threat.kind == ThreatType::Asteroid {
                    // Asteroids damage the planet shield.
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

        // 12. Laser-Threat Collision Detection:
        let mut hit_lasers = std::collections::HashSet::new();
        let mut hit_threats = std::collections::HashSet::new();
        for (l_idx, laser) in self.lasers.iter().enumerate() {
            for (t_idx, threat) in self.threats.iter().enumerate() {
                let dx = laser.x - threat.x;
                let dy = laser.y - threat.y;
                let col_dist = if laser.is_charge_shot {
                    laser.radius + threat.size
                } else {
                    threat.size + 1.5
                };
                if (dx * dx + dy * dy).sqrt() < col_dist {
                    // Only standard lasers get consumed. Charged shots pierce through.
                    if !laser.is_charge_shot {
                        hit_lasers.insert(l_idx);
                    }
                    hit_threats.insert(t_idx);
                }
            }
        }
        // Destroy hit threats, increment score, spawn visual explosions.
        let old_threats_for_hits = std::mem::take(&mut self.threats);
        let mut remaining_threats = Vec::new();
        for (idx, threat) in old_threats_for_hits.into_iter().enumerate() {
            if hit_threats.contains(&idx) {
                self.score += 10;
                self.spawn_explosion(threat.x, threat.y, 15);
            } else {
                remaining_threats.push(threat);
            }
        }
        self.threats = remaining_threats;
        let old_lasers = std::mem::take(&mut self.lasers);
        self.lasers = old_lasers
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !hit_lasers.contains(i))
            .map(|(_, l)| l)
            .collect();

        // 13. Move and fade particle effects.
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.04;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    /// Spawns explosion particles flying out radially from (x, y).
    pub fn spawn_explosion(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let (ang, spd) = (
                js_sys::Math::random() * std::f64::consts::TAU,
                js_sys::Math::random() * 1.5 + 0.5,
            );
            self.particles.push(Particle {
                x,
                y,
                vx: ang.cos() * spd,
                vy: ang.sin() * spd,
                life: 1.0,
            });
        }
    }

    /// Triggered when the fire action begins (charging weapon).
    pub fn start_charging(&mut self) {
        if self.status == GameStatus::Playing {
            self.is_charging = true;
        }
    }

    /// Triggered when the fire action ends (releasing weapon).
    /// Spawns the appropriate laser based on charge time.
    pub fn release_charge(&mut self) {
        if self.status != GameStatus::Playing || !self.is_charging {
            return;
        }
        if self.charge_level >= 2.0 {
            // Full charge: Fire massive laser beam.
            self.beam_time = 35;
            self.spawn_explosion(self.player_x, 86.0, 20);
        } else if self.charge_level >= 1.0 {
            // Medium charge: Fire a heavy piercing projectile.
            self.lasers.push(Laser {
                x: self.player_x,
                y: 86.0,
                vx: 0.0,
                vy: -1.5,
                is_charge_shot: true,
                radius: 7.5,
            });
            self.spawn_explosion(self.player_x, 86.0, 15);
        } else {
            // Tap/Low charge: Fire a rapid-fire basic laser.
            self.lasers.push(Laser {
                x: self.player_x,
                y: 88.0,
                vx: 0.0,
                vy: -2.0,
                is_charge_shot: false,
                radius: 1.0,
            });
        }
        self.is_charging = false;
        self.charge_level = 0.0;
    }

    /// Adjust horizontal coordinate of the player ship.
    pub fn move_player(&mut self, dx: f64) {
        if self.status == GameStatus::Playing {
            self.player_x = (self.player_x + dx).clamp(6.0, 94.0);
        }
    }
}
