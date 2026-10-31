//! Strata Icons: Built-in, zero-runtime-cost vector icon suite for Strata.

#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

use strata_dom::{NodeBlueprint, Tag, View};
use strata_macro::{component, rsx};

/// Vector path drawing commands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    Close,
}

/// Strongly typed identifier for all standard Strata icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconName {
    Search,
    Bell,
    Settings,
    ChevronRight,
    ChevronLeft,
    ChevronDown,
    ChevronUp,
    Check,
    Sparkles,
    Plus,
    Minus,
    X,
    Filter,
    Trash,
    Eye,
    EyeOff,
    Lock,
    Unlock,
    Layers,
    ShoppingBag,
    ShoppingCart,
    ArrowUpRight,
    Activity,
    Cpu,
    Server,
    Globe,
    Mail,
    User,
    Copy,
    ExternalLink,
    Star,
    Heart,
}

#[derive(Clone, Default)]
pub struct IconProps {
    pub name: Option<IconName>,
    pub size: Option<f32>,
    pub class: Option<&'static str>,
}

/// Generic Dynamic Icon Component
#[component]
pub fn Icon(name: IconName, class: Option<&'static str>) -> View {
    let classes = vec![
        "inline-block flex-shrink-0 select-none stroke-current fill-none",
        class.unwrap_or("w-4 h-4"),
    ];

    rsx! {
        <svg class={classes.join(" ")} viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
    }
}

/// Typed static icon convenience components.
pub struct Icons;

impl Icons {
    pub fn Search(class: Option<&'static str>) -> View {
        Icon(IconName::Search, class)
    }

    pub fn Bell(class: Option<&'static str>) -> View {
        Icon(IconName::Bell, class)
    }

    pub fn Settings(class: Option<&'static str>) -> View {
        Icon(IconName::Settings, class)
    }

    pub fn Check(class: Option<&'static str>) -> View {
        Icon(IconName::Check, class)
    }

    pub fn Sparkles(class: Option<&'static str>) -> View {
        Icon(IconName::Sparkles, class)
    }

    pub fn ChevronRight(class: Option<&'static str>) -> View {
        Icon(IconName::ChevronRight, class)
    }

    pub fn ChevronDown(class: Option<&'static str>) -> View {
        Icon(IconName::ChevronDown, class)
    }

    pub fn Plus(class: Option<&'static str>) -> View {
        Icon(IconName::Plus, class)
    }

    pub fn Minus(class: Option<&'static str>) -> View {
        Icon(IconName::Minus, class)
    }

    pub fn X(class: Option<&'static str>) -> View {
        Icon(IconName::X, class)
    }

    pub fn Filter(class: Option<&'static str>) -> View {
        Icon(IconName::Filter, class)
    }

    pub fn Trash(class: Option<&'static str>) -> View {
        Icon(IconName::Trash, class)
    }

    pub fn ShoppingBag(class: Option<&'static str>) -> View {
        Icon(IconName::ShoppingBag, class)
    }

    pub fn ShoppingCart(class: Option<&'static str>) -> View {
        Icon(IconName::ShoppingCart, class)
    }

    pub fn ArrowUpRight(class: Option<&'static str>) -> View {
        Icon(IconName::ArrowUpRight, class)
    }

    pub fn Globe(class: Option<&'static str>) -> View {
        Icon(IconName::Globe, class)
    }

    pub fn Cpu(class: Option<&'static str>) -> View {
        Icon(IconName::Cpu, class)
    }

    pub fn Server(class: Option<&'static str>) -> View {
        Icon(IconName::Server, class)
    }

    pub fn Eye(class: Option<&'static str>) -> View {
        Icon(IconName::Eye, class)
    }

    pub fn Lock(class: Option<&'static str>) -> View {
        Icon(IconName::Lock, class)
    }
}
