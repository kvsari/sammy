//! RESTful web code. Contains handlers and shared state structures.

mod handler;
mod state;

pub use self::state::State;

pub use self::handler::trade_match_root;
