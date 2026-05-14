//! TUI components

pub mod loading_spinner;
pub mod status_badge;

pub use loading_spinner::LoadingSpinner;
pub use status_badge::{status_badge, status_indicator, compact, BadgeStyle};