use core::time::Duration;

pub mod clamp;
pub use clamp::clamp;
pub mod error;
pub use error::{Error, ExecutionResult, Result};


pub enum SlotTime {
	UserSpecified(Duration),
	AutoGenerated(Duration),
}
