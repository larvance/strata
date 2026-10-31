//! Strata: Universal High-Performance Frontend Engine & Framework in Pure Rust.

pub use strata_dom as dom;
pub use strata_js as js;
pub use strata_icons as icons;
pub use strata_layout as layout;
pub use strata_macro::*;
pub use strata_platform as platform;
pub use strata_render as render;
pub use strata_style as style;
pub use strata_ui as ui;
pub use strata_primitives as primitives;

pub mod prelude {
    pub use strata_dom::{
        use_effect, use_memo, use_signal, use_state, NodeBlueprint, Signal, Tag, View,
    };
    pub use strata_js::prelude::*;
    pub use strata_macro::{component, dynamic, rsx};
    pub use strata_platform::WindowConfig;
    pub use strata_render::scene3d::{
        Camera3D, Geometry3D, Light3D, Material3D, Mesh3D, Scene3D, ShaderSource, Transform3D,
    };
    pub use strata_style::ComputedStyle;
    pub use strata_icons::Icons;
    pub use strata_ui::prelude::*;
    pub use strata_primitives::prelude::*;
}

/// Runs a Strata application with the specified root component and configuration.
pub fn launch<F>(_root: F)
where
    F: FnOnce() -> dom::View + 'static,
{
    println!("⚡ Initializing Strata Universal Frontend Runtime...");
}
