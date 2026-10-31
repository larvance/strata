//! Live Application Runtime Orchestrator connecting RSX Blueprints, Style Cascade, Layout, and Rendering.

use strata_dom::{NodeBlueprint, Tag, View};
use strata_layout::prelude::*;
use strata_render::compositor::{DisplayList, DrawCommand};
use strata_style::types::{Color, ComputedStyle, Dimension, Display, FlexDirection};

/// Live running application instance holding active state, layout tree, and display list.
#[derive(Debug)]
pub struct AppRuntime {
    pub root_blueprint: NodeBlueprint,
    pub layout_root: LayoutNode,
    pub display_list: DisplayList,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub dpr: f32,
    pub frame_count: u64,
}

impl AppRuntime {
    /// Creates and mounts a new application runtime from a root view.
    pub fn new(root_view: View, viewport_width: f32, viewport_height: f32, dpr: f32) -> Self {
        let root_blueprint = root_view.root;
        let mut layout_root = Self::build_layout_tree(&root_blueprint);

        layout_root.compute_layout(viewport_width, viewport_height);

        let mut runtime = Self {
            root_blueprint,
            layout_root,
            display_list: DisplayList::new(),
            viewport_width,
            viewport_height,
            dpr,
            frame_count: 0,
        };

        runtime.compile_display_list();
        runtime
    }

    /// Re-evaluates layout and compiles display commands for the current frame.
    pub fn tick_frame(&mut self) -> &DisplayList {
        self.frame_count += 1;
        self.layout_root.compute_layout(self.viewport_width, self.viewport_height);
        self.compile_display_list();
        &self.display_list
    }

    /// Recursively transforms an RSX `NodeBlueprint` into a `LayoutNode` hierarchy with styling.
    pub fn build_layout_tree(blueprint: &NodeBlueprint) -> LayoutNode {
        match blueprint {
            NodeBlueprint::Element {
                tag,
                classes,
                children,
                ..
            } => {
                let mut style = ComputedStyle::default();
                Self::apply_tailwind_classes(&mut style, classes);

                // Default display based on tag if not explicitly set by classes
                if classes.is_empty() {
                    match tag {
                        Tag::Div | Tag::Main | Tag::Section | Tag::Header | Tag::Footer | Tag::Nav | Tag::Aside => {
                            style.display = Display::Flex;
                            style.flex_direction = FlexDirection::Column;
                        }
                        Tag::Button => {
                            style.display = Display::Flex;
                            style.height = Dimension::Px(40.0);
                        }
                        Tag::Span | Tag::P | Tag::H1 | Tag::H2 | Tag::H3 | Tag::H4 | Tag::H5 | Tag::H6 => {
                            style.display = Display::Block;
                        }
                        _ => {
                            style.display = Display::Flex;
                        }
                    }
                }

                let mut child_nodes = Vec::with_capacity(children.len());
                for child in children {
                    child_nodes.push(Self::build_layout_tree(child));
                }

                LayoutNode::with_children(style, child_nodes)
            }
            NodeBlueprint::Text(text) => {
                let mut style = ComputedStyle::default();
                style.display = Display::Block;
                // Intrinsic size heuristic for text: ~8px per character width, 20px line height
                let approx_width = (text.len() as f32 * 8.0).max(16.0);
                style.width = Dimension::Px(approx_width);
                style.height = Dimension::Px(20.0);
                LayoutNode::new(style)
            }
            NodeBlueprint::DynamicText(dyn_fn) => {
                let text = dyn_fn();
                let mut style = ComputedStyle::default();
                style.display = Display::Block;
                let approx_width = (text.len() as f32 * 8.0).max(16.0);
                style.width = Dimension::Px(approx_width);
                style.height = Dimension::Px(20.0);
                LayoutNode::new(style)
            }
            NodeBlueprint::Fragment(children) => {
                let mut style = ComputedStyle::default();
                style.display = Display::Flex;
                style.flex_direction = FlexDirection::Column;

                let mut child_nodes = Vec::with_capacity(children.len());
                for child in children {
                    child_nodes.push(Self::build_layout_tree(child));
                }

                LayoutNode::with_children(style, child_nodes)
            }
            NodeBlueprint::Empty => {
                let mut style = ComputedStyle::default();
                style.display = Display::None;
                LayoutNode::new(style)
            }
        }
    }

    /// Basic fast Tailwind class parser mapping common classes to layout dimensions and display modes.
    fn apply_tailwind_classes(style: &mut ComputedStyle, classes: &[String]) {
        for cls in classes {
            match cls.as_str() {
                "flex" => style.display = Display::Flex,
                "grid" => style.display = Display::Grid,
                "block" => style.display = Display::Block,
                "hidden" => style.display = Display::None,
                "flex-row" => style.flex_direction = FlexDirection::Row,
                "flex-col" => style.flex_direction = FlexDirection::Column,
                "flex-1" => style.flex_grow = 1.0,
                "h-screen" => style.height = Dimension::Percent(100.0),
                "w-screen" | "w-full" => style.width = Dimension::Percent(100.0),
                "h-full" => style.height = Dimension::Percent(100.0),
                "overflow-hidden" => {}
                _ => {}
            }
        }
    }

    /// Generates draw commands for all computed layout rectangles.
    fn compile_display_list(&mut self) {
        self.display_list.clear();
        Self::emit_display_commands(&self.layout_root, &mut self.display_list);
    }

    fn emit_display_commands(node: &LayoutNode, dl: &mut DisplayList) {
        let rect = node.geometry.border_box;
        if rect.width > 0.0 && rect.height > 0.0 {
            dl.push(DrawCommand::DrawRect {
                bounds: rect,
                color: Color::rgba(25, 25, 25, 255), // Neutral base
            });
        }

        for child in &node.children {
            Self::emit_display_commands(child, dl);
        }
    }
}

/// Launches a live Strata application with the specified root component.
pub fn launch<F>(root: F)
where
    F: FnOnce() -> View + 'static,
{
    println!("⚡ Initializing Strata Universal Frontend Runtime...");
    let view = root();
    let runtime = AppRuntime::new(view, 1280.0, 800.0, 1.0);
    println!(
        "✔ Strata Runtime active: viewport {}x{}, rendered {} display commands.",
        runtime.viewport_width,
        runtime.viewport_height,
        runtime.display_list.commands.len()
    );
}

/// Headless runner returning the initialized `AppRuntime` for testing and layout inspection.
pub fn launch_headless<F>(root: F, width: f32, height: f32) -> AppRuntime
where
    F: FnOnce() -> View + 'static,
{
    let view = root();
    AppRuntime::new(view, width, height, 1.0)
}
