//! Per-tick game-state update.
//!
//! The orchestrator is [`GameState::update`] in this file. Each phase
//! is split into a single private method defined in
//! [`super::early_phases`] (1-6) or [`super::late_phases`] (7-13);
//! the dispatching sequence is documented in the orchestrator itself.

use super::types::*;

mod early_phases;
mod late_phases;

impl GameState {
    /// Run all 13 update phases for a single game tick.
    pub fn update(&mut self) {
        self.ticks += 1;
        self.advance_stars();
        self.update_helper_weapons();
        self.update_beam_power_shot();
        self.update_powerups();
        self.update_charging();
        self.update_boss();
        self.update_spawning();
        self.update_wave_progression();
        self.move_projectiles();
        self.move_threats();
        self.handle_threat_collisions();
        self.handle_collisions();
        self.update_particles();
    }
}
