pub mod updates;

pub use updates::{Order, render_updates};

pub(crate) const MILESTONE_REGEX: &'static str = r"^\d{4}([hH][12])?$";
