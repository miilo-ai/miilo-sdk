//! Pure observation runtime. Hosts supply the clock, epoch and provider adapters.
mod error;
mod explanation;
pub mod observation;
mod state;
mod validation;
pub use error::RuntimeError;
pub use explanation::{GroundedExplanation, explain};
pub use state::ObservatoryRuntime;
pub use validation::validate_snapshot;
