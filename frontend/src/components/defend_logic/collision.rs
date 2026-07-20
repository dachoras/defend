use super::types::*;
use std::collections::HashSet;

impl GameState {
    pub fn handle_collisions(&mut self) {
        let mut hit_lasers = HashSet::new();
        let mut hit_threats = HashSet::new();
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
        self.lasers = old_lasers
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !hit_lasers.contains(i))
            .map(|(_, l)| l)
            .collect();
    }
}
