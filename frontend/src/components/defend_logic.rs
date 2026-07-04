//! Defend game logic and grid state management.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum GameStatus { NotStarted, Playing, Lost }

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum ThreatType { Asteroid, Scout, Bullet }

#[derive(Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub struct Laser { pub x: f64, pub y: f64, pub vx: f64, pub vy: f64, pub is_charge_shot: bool, pub radius: f64 }

#[derive(Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub struct Threat { pub x: f64, pub y: f64, pub speed: f64, pub size: f64, pub kind: ThreatType }

#[derive(Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub struct Particle { pub x: f64, pub y: f64, pub vx: f64, pub vy: f64, pub life: f64 }

#[derive(Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub struct Star { pub x: f64, pub y: f64, pub speed: f64, pub size: f64 }

#[derive(Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub struct GameState {
    pub player_x: f64, pub lasers: Vec<Laser>, pub threats: Vec<Threat>, pub particles: Vec<Particle>,
    pub score: u32, pub player_shield: u32, pub planet_shield: u32, pub wave: u32, pub status: GameStatus, pub ticks: u64,
    pub charge_level: f64, pub is_charging: bool, pub stars: Vec<Star>,
    pub powerup_x: f64, pub powerup_y: f64, pub powerup_type: u8, pub helper_time: u32,
    pub beam_time: u32, pub boss_health: Option<u32>, pub boss_max_health: u32, pub boss_x: f64, pub boss_vx: f64,
}

impl GameState {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            player_x: 50.0, lasers: Vec::new(), threats: Vec::new(), particles: Vec::new(),
            score: 0, player_shield: 100, planet_shield: 100, wave: 1, status: GameStatus::NotStarted, ticks: 0,
            charge_level: 0.0, is_charging: false,
            stars: (0..22).map(|_| Star {
                x: js_sys::Math::random() * 100.0,
                y: js_sys::Math::random() * 100.0,
                speed: js_sys::Math::random() * 0.45 + 0.15,
                size: js_sys::Math::random() * 0.45 + 0.15,
            }).collect(),
            powerup_x: 0.0, powerup_y: 0.0, powerup_type: 0, helper_time: 0,
            beam_time: 0, boss_health: None, boss_max_health: 100, boss_x: 0.0, boss_vx: 0.0,
        }
    }

    #[rustfmt::skip]
    pub fn start(&mut self) { *self = Self::new(); self.status = GameStatus::Playing; }

    #[rustfmt::skip]
    pub fn update(&mut self) {
        if self.status != GameStatus::Playing { return; }
        self.ticks += 1;
        for s in &mut self.stars {
            s.y += s.speed;
            if s.y > 100.0 { s.y = 0.0; s.x = js_sys::Math::random() * 100.0; }
        }
        if self.helper_time > 0 {
            self.helper_time -= 1;
            if self.helper_time % 18 == 0 && !self.threats.is_empty() {
                let (mut target_idx, mut min_dy) = (0, f64::MAX);
                for (i, t) in self.threats.iter().enumerate() {
                    let dy = 92.0 - t.y; if dy > 0.0 && dy < min_dy { min_dy = dy; target_idx = i; }
                }
                let t = &self.threats[target_idx];
                for lx in [self.player_x - 5.5, self.player_x + 5.5] {
                    let (dx, dy) = (t.x - lx, t.y - 92.0);
                    let len = (dx * dx + dy * dy).sqrt().max(1.0);
                    self.lasers.push(Laser { x: lx, y: 92.0, vx: (dx / len) * 2.2, vy: (dy / len) * 2.2, is_charge_shot: false, radius: 0.8 });
                }
            }
        }
        if self.beam_time > 0 {
            self.beam_time -= 1;
            let bx = self.player_x;
            let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
            let old = std::mem::take(&mut self.threats);
            let mut rem = Vec::new();
            for t in old {
                if (t.x - bx).abs() < limit && t.y < 88.0 {
                    self.score += 15; self.spawn_explosion(t.x, t.y, 8);
                } else { rem.push(t); }
            }
            self.threats = rem;
            if self.ticks % 2 == 0 {
                let py = js_sys::Math::random() * 80.0;
                self.particles.push(Particle {
                    x: bx + js_sys::Math::random() * 2.0 - 1.0, y: py,
                    vx: js_sys::Math::random() * 0.8 - 0.4, vy: js_sys::Math::random() * 0.8 - 0.4,
                    life: 0.5,
                });
            }
        }
        if self.powerup_type > 0 {
            self.powerup_y += 0.55;
            if self.powerup_y > 100.0 { self.powerup_type = 0; }
            else if self.powerup_y >= 90.0 && self.powerup_y <= 95.0 && (self.powerup_x - self.player_x).abs() < 5.0 {
                if self.powerup_type == 1 {
                    self.player_shield = (self.player_shield + 25).min(100);
                } else { self.helper_time = 450; }
                self.spawn_explosion(self.powerup_x, self.powerup_y, 10);
                self.powerup_type = 0;
            }
        } else if self.ticks % 300 == 0 {
            self.powerup_x = js_sys::Math::random() * 80.0 + 10.0; self.powerup_y = 0.0;
            self.powerup_type = if js_sys::Math::random() > 0.5 { 1 } else { 2 };
        }
        if self.is_charging {
            self.charge_level = (self.charge_level + 0.025).min(2.0);
            if self.ticks % 2 == 0 {
                let angle = js_sys::Math::random() * std::f64::consts::TAU;
                let dist = js_sys::Math::random() * 8.0 + 4.0;
                let px = self.player_x + angle.cos() * dist;
                let py = 87.0 + angle.sin() * dist;
                self.particles.push(Particle { x: px, y: py, vx: (self.player_x - px) * 0.12, vy: (87.0 - py) * 0.12, life: 0.8 });
            }
        }
        if let Some(mut bh) = self.boss_health {
            self.boss_x += self.boss_vx;
            if self.boss_x > 90.0 || self.boss_x < 10.0 { self.boss_vx = -self.boss_vx; }
            if self.ticks % 40 == 0 { self.threats.push(Threat { x: self.boss_x, y: 17.0, speed: 1.2, size: 1.0, kind: ThreatType::Bullet }); }
            let mut hit_lasers = std::collections::HashSet::new();
            let mut explosions = Vec::new();
            for (idx, l) in self.lasers.iter().enumerate() {
                if (l.x - self.boss_x).abs() < 8.0 && (l.y - 15.0).abs() < 5.0 {
                    bh = bh.saturating_sub(if l.is_charge_shot { 50 } else { 10 });
                    explosions.push((l.x, l.y)); hit_lasers.insert(idx);
                }
            }
            for (ex, ey) in explosions { self.spawn_explosion(ex, ey, 6); }
            if self.beam_time > 0 {
                let limit = if self.helper_time > 0 { 8.5 } else { 5.0 };
                if (self.boss_x - self.player_x).abs() < limit { bh = bh.saturating_sub(2); }
            }
            if bh == 0 {
                self.boss_health = None; self.score += 500; self.spawn_explosion(self.boss_x, 15.0, 40); self.wave += 1;
            } else { self.boss_health = Some(bh); }
            let old_lasers = std::mem::take(&mut self.lasers);
            self.lasers = old_lasers.into_iter().enumerate().filter(|(i, _)| !hit_lasers.contains(i)).map(|(_, l)| l).collect();
        }
        if self.boss_health.is_none() {
            let spawn_interval = (35 - (self.wave as i32 * 2)).max(10) as u64;
            if self.ticks.is_multiple_of(spawn_interval) {
                let (x, is_s) = (js_sys::Math::random() * 90.0 + 5.0, js_sys::Math::random() > 0.5);
                let (spd, sz, k) = if is_s { (js_sys::Math::random() * 0.35 + 0.45 + (self.wave as f64 * 0.05), 2.2, ThreatType::Scout) } else { (js_sys::Math::random() * 0.25 + 0.25 + (self.wave as f64 * 0.04), 3.0, ThreatType::Asteroid) };
                self.threats.push(Threat { x, y: 0.0, speed: spd, size: sz, kind: k });
            }
            if self.ticks % 75 == 0 {
                if let Some(t) = self.threats.iter().find(|t| t.kind == ThreatType::Scout && t.y > 10.0 && t.y < 70.0) {
                    let bx = t.x; let by = t.y + 2.0; let bspd = t.speed + 0.6;
                    self.threats.push(Threat { x: bx, y: by, speed: bspd, size: 0.9, kind: ThreatType::Bullet });
                }
            }
        }
        if self.ticks.is_multiple_of(600) && self.boss_health.is_none() {
            self.wave += 1;
            if self.wave % 10 == 0 {
                let hp = 150 + self.wave * 10;
                self.boss_health = Some(hp); self.boss_max_health = hp; self.boss_x = 50.0; self.boss_vx = 0.8;
                self.threats.clear();
            }
        }
        for laser in &mut self.lasers { laser.x += laser.vx; laser.y += laser.vy; }
        self.lasers.retain(|l| l.y > 0.0 && l.y < 100.0 && l.x > 0.0 && l.x < 100.0);
        for threat in &mut self.threats { threat.y += threat.speed; }

        let old_threats = std::mem::take(&mut self.threats);
        let mut new_threats = Vec::new();
        for mut threat in old_threats {
            if threat.y >= 90.0 && threat.y <= 95.0 && (threat.x - self.player_x).abs() < 5.0 {
                let dmg = if threat.kind == ThreatType::Bullet { 15 } else { 20 };
                self.player_shield = self.player_shield.saturating_sub(dmg);
                self.spawn_explosion(threat.x, threat.y, 8);
                if self.player_shield == 0 { self.status = GameStatus::Lost; }
            } else if threat.y >= 100.0 {
                if threat.kind == ThreatType::Scout {
                    threat.y = 0.0; threat.x = js_sys::Math::random() * 90.0 + 5.0;
                    new_threats.push(threat);
                } else if threat.kind == ThreatType::Asteroid {
                    self.planet_shield = self.planet_shield.saturating_sub(10);
                    if self.planet_shield == 0 { self.status = GameStatus::Lost; }
                }
            } else { new_threats.push(threat); }
        }
        self.threats = new_threats;

        let mut hit_lasers = std::collections::HashSet::new();
        let mut hit_threats = std::collections::HashSet::new();
        for (l_idx, laser) in self.lasers.iter().enumerate() {
            for (t_idx, threat) in self.threats.iter().enumerate() {
                let dx = laser.x - threat.x; let dy = laser.y - threat.y;
                let col_dist = if laser.is_charge_shot { laser.radius + threat.size } else { threat.size + 1.5 };
                if (dx*dx + dy*dy).sqrt() < col_dist {
                    if !laser.is_charge_shot { hit_lasers.insert(l_idx); }
                    hit_threats.insert(t_idx);
                }
            }
        }
        let old_threats_for_hits = std::mem::take(&mut self.threats);
        let mut remaining_threats = Vec::new();
        for (idx, threat) in old_threats_for_hits.into_iter().enumerate() {
            if hit_threats.contains(&idx) {
                self.score += 10; self.spawn_explosion(threat.x, threat.y, 15);
            } else { remaining_threats.push(threat); }
        }
        self.threats = remaining_threats;
        let old_lasers = std::mem::take(&mut self.lasers);
        self.lasers = old_lasers.into_iter().enumerate().filter(|(i, _)| !hit_lasers.contains(i)).map(|(_, l)| l).collect();

        for p in &mut self.particles { p.x += p.vx; p.y += p.vy; p.life -= 0.04; }
        self.particles.retain(|p| p.life > 0.0);
    }

    #[rustfmt::skip]
    pub fn spawn_explosion(&mut self, x: f64, y: f64, count: usize) {
        for _ in 0..count {
            let (ang, spd) = (js_sys::Math::random() * std::f64::consts::TAU, js_sys::Math::random() * 1.5 + 0.5);
            self.particles.push(Particle { x, y, vx: ang.cos() * spd, vy: ang.sin() * spd, life: 1.0 });
        }
    }

    #[rustfmt::skip]
    pub fn start_charging(&mut self) { if self.status == GameStatus::Playing { self.is_charging = true; } }

    #[rustfmt::skip]
    pub fn release_charge(&mut self) {
        if self.status != GameStatus::Playing || !self.is_charging { return; }
        if self.charge_level >= 2.0 {
            self.beam_time = 35; self.spawn_explosion(self.player_x, 86.0, 20);
        } else if self.charge_level >= 1.0 {
            self.lasers.push(Laser { x: self.player_x, y: 86.0, vx: 0.0, vy: -1.5, is_charge_shot: true, radius: 7.5 });
            self.spawn_explosion(self.player_x, 86.0, 15);
        } else {
            self.lasers.push(Laser { x: self.player_x, y: 88.0, vx: 0.0, vy: -2.0, is_charge_shot: false, radius: 1.0 });
        }
        self.is_charging = false; self.charge_level = 0.0;
    }

    #[rustfmt::skip]
    pub fn move_player(&mut self, dx: f64) { if self.status == GameStatus::Playing { self.player_x = (self.player_x + dx).clamp(6.0, 94.0); } }
}
