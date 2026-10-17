//! Pure observation runtime. Hosts supply the clock, epoch and provider adapters.
mod explanation;
pub mod observation;
mod state;
mod validation;
pub use explanation::{GroundedExplanation, explain};
pub use state::{ObservatoryRuntime, RuntimeError};
pub use validation::validate_snapshot;
