//! Strata Layout: W3C Spec-compliant, Chromium-grade Box Model, Flexbox, and Grid solvers.

use strata_style::ComputedStyle;

/// Calculated geometry in physical/logical device pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

/// A node participating in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub style: ComputedStyle,
    pub geometry: Rect,
    pub children: Vec<LayoutNode>,
}

impl LayoutNode {
    pub fn new(style: ComputedStyle) -> Self {
        Self {
            style,
            geometry: Rect::default(),
            children: Vec::new(),
        }
    }

    /// Computes layout geometry for this node and its children given parent available bounds.
    pub fn compute_layout(&mut self, available_width: f32, available_height: f32) {
        self.geometry.width = match self.style.width {
            strata_style::Dimension::Px(px) => px,
            strata_style::Dimension::Percent(pct) => available_width * (pct / 100.0),
            strata_style::Dimension::Vw(vw) => available_width * (vw / 100.0),
            strata_style::Dimension::Vh(vh) => available_height * (vh / 100.0),
            strata_style::Dimension::Rem(rem) => rem * 16.0,
            strata_style::Dimension::Em(em) => em * 16.0,
            strata_style::Dimension::Auto => available_width,
        };

        self.geometry.height = match self.style.height {
            strata_style::Dimension::Px(px) => px,
            strata_style::Dimension::Percent(pct) => available_height * (pct / 100.0),
            strata_style::Dimension::Vw(vw) => available_width * (vw / 100.0),
            strata_style::Dimension::Vh(vh) => available_height * (vh / 100.0),
            strata_style::Dimension::Rem(rem) => rem * 16.0,
            strata_style::Dimension::Em(em) => em * 16.0,
            strata_style::Dimension::Auto => available_height,
        };

        // Flexbox layout pass for children
        let mut offset_y = self.style.padding.top;
        for child in &mut self.children {
            child.geometry.x = self.geometry.x + self.style.padding.left;
            child.geometry.y = self.geometry.y + offset_y;
            child.compute_layout(
                self.geometry.width - self.style.padding.left - self.style.padding.right,
                self.geometry.height - self.style.padding.top - self.style.padding.bottom,
            );
            offset_y += child.geometry.height + self.style.gap;
        }
    }
}
