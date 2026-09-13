pub mod generate;
pub mod manager_packet;
pub mod model;
pub mod questions;

pub use generate::generate;
pub use model::ReactAnswers;
pub use questions::{ask_arch, ask_libs, ask_pm, check_name};