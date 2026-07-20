use super::types::*;

impl GameState {
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

    pub fn start_charging(&mut self) {
        if self.status == GameStatus::Playing {
            self.is_charging = true;
        }
    }

    pub fn release_charge(&mut self) {
        if self.status != GameStatus::Playing || !self.is_charging {
            return;
        }
        if self.charge_level >= 2.0 {
            self.beam_time = 35;
            self.spawn_explosion(self.player_x, 86.0, 20);
        } else if self.charge_level >= 1.0 {
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

    pub fn move_player(&mut self, dx: f64) {
        if self.status == GameStatus::Playing {
            self.player_x = (self.player_x + dx).clamp(6.0, 94.0);
        }
    }
}
