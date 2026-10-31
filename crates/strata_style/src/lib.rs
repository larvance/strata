//! Strata Style: CSS spec parser, cascade solver, and Tailwind JIT compiler.

/// CSS Specificity tuple (Inline, ID, Class/Attribute/Pseudo, Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Specificity {
    pub inline: u8,
    pub id_count: u16,
    pub class_count: u16,
    pub type_count: u16,
}

/// Resolved computed style for an element.
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub display: Display,
    pub width: Dimension,
    pub height: Dimension,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub background_color: Option<Color>,
    pub border_radius: f32,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub gap: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Block,
    Inline,
    Flex,
    Grid,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlignItems {
    #[default]
    Stretch,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Dimension {
    #[default]
    Auto,
    Px(f32),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Tailwind CSS JIT compiler and resolver.
pub struct TailwindEngine;

impl TailwindEngine {
    pub fn resolve_classes(classes: &str) -> ComputedStyle {
        let mut style = ComputedStyle::default();
        for class in classes.split_whitespace() {
            Self::apply_class(&mut style, class);
        }
        style
    }

    fn apply_class(style: &mut ComputedStyle, class: &str) {
        match class {
            "flex" => style.display = Display::Flex,
            "grid" => style.display = Display::Grid,
            "hidden" => style.display = Display::None,
            "flex-col" => style.flex_direction = FlexDirection::Column,
            "flex-row" => style.flex_direction = FlexDirection::Row,
            "items-center" => style.align_items = AlignItems::Center,
            "justify-between" => style.justify_content = JustifyContent::SpaceBetween,
            "justify-center" => style.justify_content = JustifyContent::Center,
            "p-4" => {
                style.padding = EdgeInsets { top: 16.0, right: 16.0, bottom: 16.0, left: 16.0 };
            }
            "px-4" => {
                style.padding.left = 16.0;
                style.padding.right = 16.0;
            }
            "py-2" => {
                style.padding.top = 8.0;
                style.padding.bottom = 8.0;
            }
            "gap-2" => style.gap = 8.0,
            "gap-4" => style.gap = 16.0,
            "rounded-lg" => style.border_radius = 8.0,
            "rounded-2xl" => style.border_radius = 16.0,
            _ => {}
        }
    }
}
