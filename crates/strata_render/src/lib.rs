//! Strata Render: Display List, Stacking Context Compositor, GPU rasterizer, and Unified 3D Engine.

use strata_layout::Rect;
use strata_style::Color;

pub mod scene3d;

/// Low-level drawing primitives emitted to GPU command buffers.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    DrawRect {
        bounds: Rect,
        color: Color,
    },
    DrawRoundedRect {
        bounds: Rect,
        color: Color,
        radius: f32,
    },
    DrawShadow {
        bounds: Rect,
        blur: f32,
        spread: f32,
        color: Color,
    },
    DrawText {
        origin: (f32, f32),
        text: String,
        font_size: f32,
        color: Color,
    },
    DrawViewport3D {
        bounds: Rect,
        scene: scene3d::Scene3D,
    },
    ClipRect(Rect),
    PopClip,
}

/// A serialized sequence of render instructions for a single frame.
#[derive(Debug, Default, Clone)]
pub struct DisplayList {
    commands: Vec<DrawCommand>,
}

impl DisplayList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
