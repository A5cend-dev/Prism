//! Output formatting for CLI reports.

use prism_core::types::{
    report::DiagnosticReport,
    trace::{DiffChangeType, ExecutionTrace, ResourceProfile, StateDiff},
};

pub mod compact;
pub mod human;
pub mod json;
pub mod compact;
pub mod renderers;
