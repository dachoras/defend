//! Yew components used by the Defend UI.
//!
//! Grouped by purpose:
//!
//! - [`header`] / [`footer`] — re-exports of the shared header and footer
//!   components that live in `shared-frontend`. Kept as local modules so
//!   consumers can `use crate::components::header::Header` without having
//!   to learn the upstream crate path.
//! - [`pin`] — PIN-gate login form.
//! - [`event_listener`] — RAII wrapper around `addEventListener` that
//!   removes the listener on drop.
//! - [`defend`] — the game itself: board, dpad, leaderboard, overlay,
//!   game logic, and the centralised state hook.
//! - [`defend_game`] — top-level game component that composes the defend
//!   sub-modules and renders the score / overlay layout.

pub mod defend_board;
pub mod defend_controls;
pub mod defend_game;
pub mod defend_logic;
pub mod defend_overlay;
pub mod footer;
pub mod header;
pub mod pin;
