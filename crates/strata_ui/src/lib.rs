//! Strata UI: High-Level Accessible Component Library and Design System.

#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

pub mod buttons;
pub mod data_display;
pub mod feedback;
pub mod inputs;

pub mod prelude {
    pub use super::buttons::*;
    pub use super::data_display::*;
    pub use super::feedback::*;
    pub use super::inputs::*;
}
