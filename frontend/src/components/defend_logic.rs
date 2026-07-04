//! Defend game logic and grid state management.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum GameStatus {
    NotStarted,
    Playing,
    Lost,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Laser {
    pub x: f64,
    pub y: f64,
    pub is_charge_shot: bool,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Threat {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub player_x: f64,
    pub lasers: Vec<Laser>,
    pub threats: Vec<Threat>,
    pub particles: Vec<Particle>,
    pub score: u32,
    pub shield: u32,
    pub wave: u32,
    pub status: GameStatus,
    pub ticks: u64,
    pub charge_level: f64,
    pub is_charging: bool,
}

impl GameState {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            player_x: 50.0, lasers: Vec::new(), threats: Vec::new(), particles: Vec::new(),
            score: 0, shield: 100, wave: 1, status: GameStatus::NotStarted, ticks: 0,
            charge_level: 0.0, is_charging: false,
        }
    }

    #[rustfmt::skip]
    pub fn start(&mut self) { *self = Self::new(); self.status = GameStatus::Playing; }

    pub fn update(&mut self) {
        if self.status != GameStatus::Playing {
            return;
        }

        self.ticks += 1;

        // Update charge shot status
        if self.is_charging {
            self.charge_level = (self.charge_level + 0.025).min(1.0);
            if self.ticks % 2 == 0 {
                let angle = js_sys::Math::random() * std::f64::consts::TAU;
                let dist = js_sys::Math::random() * 8.0 + 4.0;
                let px = self.player_x + angle.cos() * dist;
                let py = 87.0 + angle.sin() * dist;
                let vx = (self.player_x - px) * 0.12;
                let vy = (87.0 - py) * 0.12;
                self.particles.push(Particle {
                    x: px,
                    y: py,
                    vx,
                    vy,
                    life: 0.8,
                });
            }
        }

        // 1. Spawn threats
        let spawn_interval = (35 - (self.wave as i32 * 2)).max(10) as u64;
        if self.ticks.is_multiple_of(spawn_interval) {
            let x = js_sys::Math::random() * 90.0 + 5.0;
            let speed = js_sys::Math::random() * 0.4 + 0.3 + (self.wave as f64 * 0.05);
            let size = js_sys::Math::random() * 2.0 + 2.0;
            self.threats.push(Threat {
                x,
                y: 0.0,
                speed,
                size,
            });
        }

        // Increase wave every 600 ticks
        if self.ticks.is_multiple_of(600) {
            self.wave += 1;
        }

        // 2. Move lasers
        for laser in &mut self.lasers {
            if laser.is_charge_shot {
                laser.y -= 1.5;
            } else {
                laser.y -= 2.0;
            }
        }
        self.lasers.retain(|l| l.y > 0.0);

        // 3. Move threats
        for threat in &mut self.threats {
            threat.y += threat.speed;
        }

        // Check player and base shield collisions
        let old_threats = std::mem::take(&mut self.threats);
        let mut new_threats = Vec::new();
        for threat in old_threats {
            if threat.y >= 90.0 && threat.y <= 94.0 && (threat.x - self.player_x).abs() < 6.0 {
                // Collided with player ship
                self.shield = self.shield.saturating_sub(20);
                self.spawn_explosion(threat.x, threat.y, 10);
                if self.shield == 0 {
                    self.status = GameStatus::Lost;
                }
            } else if threat.y >= 100.0 {
                // Slipped past player, hits orbital station shields
                self.shield = self.shield.saturating_sub(5);
                if self.shield == 0 {
                    self.status = GameStatus::Lost;
                }
            } else {
                new_threats.push(threat);
            }
        }
        self.threats = new_threats;

        // 4. Laser vs Threat collisions
        let mut hit_lasers = std::collections::HashSet::new();
        let mut hit_threats = std::collections::HashSet::new();

        for (l_idx, laser) in self.lasers.iter().enumerate() {
            for (t_idx, threat) in self.threats.iter().enumerate() {
                let dx = laser.x - threat.x;
                let dy = laser.y - threat.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let col_dist = if laser.is_charge_shot {
                    laser.radius + threat.size
                } else {
                    threat.size + 1.5
                };
                if dist < col_dist {
                    if !laser.is_charge_shot {
                        hit_lasers.insert(l_idx);
                    }
                    hit_threats.insert(t_idx);
                }
            }
        }

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
        let mut remaining_lasers = Vec::new();
        for (idx, laser) in old_lasers.into_iter().enumerate() {
            if !hit_lasers.contains(&idx) {
                remaining_lasers.push(laser);
            }
        }
        self.lasers = remaining_lasers;

        // 5. Particles update
        for p in &mut self.particles {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.04;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn spawn_explosion(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let angle = js_sys::Math::random() * std::f64::consts::TAU;
            let speed = js_sys::Math::random() * 1.5 + 0.5;
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;
            self.particles.push(Particle {
                x,
                y,
                vx,
                vy,
                life: 1.0,
            });
        }
    }

    #[rustfmt::skip]
    pub fn start_charging(&mut self) { if self.status == GameStatus::Playing { self.is_charging = true; } }

    pub fn release_charge(&mut self) {
        if self.status != GameStatus::Playing || !self.is_charging {
            return;
        }
        if self.charge_level >= 1.0 {
            self.lasers.push(Laser {
                x: self.player_x,
                y: 86.0,
                is_charge_shot: true,
                radius: 7.5,
            });
            self.spawn_explosion(self.player_x, 86.0, 15);
        } else {
            self.lasers.push(Laser {
                x: self.player_x,
                y: 88.0,
                is_charge_shot: false,
                radius: 1.0,
            });
        }
        self.is_charging = false;
        self.charge_level = 0.0;
    }

    #[rustfmt::skip]
    pub fn move_player(&mut self, dx: f64) { if self.status == GameStatus::Playing { self.player_x = (self.player_x + dx).clamp(6.0, 94.0); } }
}
