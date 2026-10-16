//! Widget operator core: pure functions the controller and webhook call.
pub mod admission;
pub mod controller;
pub mod crd;
pub mod reconcile;
pub mod status;

pub use crd::{Widget, WidgetSpec, WidgetStatus};
