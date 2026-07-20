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
}
