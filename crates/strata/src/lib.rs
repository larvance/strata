//! Strata: Universal High-Performance Frontend Engine & Framework in Pure Rust.

pub use strata_dom;
pub use strata_dom as dom;
pub use strata_js;
pub use strata_js as js;
pub use strata_icons as icons;
pub use strata_layout as layout;
pub use strata_macro::*;
pub use strata_platform as platform;
pub use strata_device as device;
pub use strata_render as render;
pub use strata_style as style;
pub use strata_ui as ui;
pub use strata_primitives as primitives;
pub use strata_vfs as fs;
pub use strata_storage as storage;
pub use strata_web_api as web_api;
pub use strata_parallel as parallel;
pub use strata_tui as tui;

pub mod prelude {
    pub use strata_dom;
    pub use strata_js;
    pub use strata_dom::{
        use_effect, use_memo, use_signal, use_state, Attribute, Event, EventType, IntoNodeBlueprint,
        NodeBlueprint, Signal, Tag, View,
    };
    pub use strata_js::{
        json::{parse_json, stringify_json},
        promise::{drain_microtasks, pending_microtasks_count, queue_microtask, JsPromise, PromiseState},
        web::{
            advance_timers_by, clear_interval, clear_timeout, local_storage, session_storage,
            set_interval, set_timeout, tick_timers, Storage, TimerId, URLSearchParams,
        },
        JsObject, JsValue,
    };
    pub use strata_macro::{component, dynamic, rsx};
    pub use strata_platform::prelude::*;
    pub use strata_device::prelude::*;
    pub use strata_render::scene3d::{
        AmbientLight, BoxGeometry, Camera3D, Canvas3D, DirectionalLight, Geometry3D, Light3D,
        Material3D, Mesh, Mesh3D, PerspectiveCamera, Scene3D, ShaderMaterial, ShaderSource,
        SphereGeometry, StandardMaterial, Transform3D,
    };
    pub use strata_style::ComputedStyle;
    pub use strata_icons::Icons;
    pub use strata_ui::prelude::*;
    pub use strata_primitives::prelude::*;
    pub use strata_vfs::prelude::*;
    pub use strata_storage::prelude::*;
    pub use strata_web_api::prelude::*;
    pub use strata_parallel::prelude::*;
    pub use strata_tui::prelude::*;
}

/// Runs a Strata application with the specified root component and configuration.
pub fn launch<F>(_root: F)
where
    F: FnOnce() -> dom::View + 'static,
{
    println!("⚡ Initializing Strata Universal Frontend Runtime...");
}
