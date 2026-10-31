//! State model for Strata Creative Studio.

use strata::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Select,
    Frame,
    Rectangle,
    Ellipse,
    Text,
    Pan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub id: String,
    pub name: String,
    pub shape_type: &'static str,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill_color: String,
    pub corner_radius: f32,
    pub opacity: f32,
    pub is_locked: bool,
    pub is_visible: bool,
}

#[derive(Clone)]
pub struct StudioState {
    pub active_tool: Signal<Tool>,
    pub zoom_level: Signal<f32>,
    pub pan_offset: Signal<(f32, f32)>,
    pub selected_shape_id: Signal<Option<String>>,
    pub shapes: Signal<Vec<Shape>>,
}

impl StudioState {
    pub fn new() -> Self {
        let initial_shapes = vec![
            Shape {
                id: "layer-1".into(),
                name: "Main Artboard".into(),
                shape_type: "frame",
                x: 120.0,
                y: 80.0,
                width: 480.0,
                height: 320.0,
                fill_color: "#18181b".into(),
                corner_radius: 16.0,
                opacity: 1.0,
                is_locked: false,
                is_visible: true,
            },
            Shape {
                id: "layer-2".into(),
                name: "Metric Card Container".into(),
                shape_type: "rectangle",
                x: 152.0,
                y: 112.0,
                width: 200.0,
                height: 120.0,
                fill_color: "#27272a".into(),
                corner_radius: 12.0,
                opacity: 0.95,
                is_locked: false,
                is_visible: true,
            },
            Shape {
                id: "layer-3".into(),
                name: "Accent Status Indicator".into(),
                shape_type: "ellipse",
                x: 172.0,
                y: 132.0,
                width: 32.0,
                height: 32.0,
                fill_color: "#6366f1".into(),
                corner_radius: 16.0,
                opacity: 1.0,
                is_locked: false,
                is_visible: true,
            },
        ];

        Self {
            active_tool: use_signal(Tool::Select),
            zoom_level: use_signal(100.0),
            pan_offset: use_signal((0.0, 0.0)),
            selected_shape_id: use_signal(Some("layer-2".into())),
            shapes: use_signal(initial_shapes),
        }
    }

    pub fn selected_shape(&self) -> Option<Shape> {
        let id = self.selected_shape_id.get()?;
        self.shapes.get().into_iter().find(|s| s.id == id)
    }
}
