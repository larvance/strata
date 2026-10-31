//! Feedback and Overlay Components for Strata UI.

#![allow(non_snake_case)]

use strata_dom::{NodeBlueprint, Signal, View};
use strata_macro::{component, rsx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Destructive,
    Outline,
}

#[component]
pub fn Badge(
    variant: Option<BadgeVariant>,
    class: Option<&'static str>,
) -> View {
    let var = variant.unwrap_or_default();
    let var_classes = match var {
        BadgeVariant::Default => "bg-neutral-800 text-neutral-300 border-neutral-700",
        BadgeVariant::Success => "bg-emerald-500/10 text-emerald-400 border-emerald-500/20",
        BadgeVariant::Warning => "bg-amber-500/10 text-amber-400 border-amber-500/20",
        BadgeVariant::Destructive => "bg-rose-500/10 text-rose-400 border-rose-500/20",
        BadgeVariant::Outline => "bg-transparent text-white border-neutral-700",
    };

    let custom = class.unwrap_or("");
    let combined = format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-mono font-medium border {var_classes} {custom}");

    rsx! {
        <span class={combined}>
            {"Badge"}
        </span>
    }
}

#[component]
pub fn Modal(
    open: Signal<bool>,
    title: &'static str,
    class: Option<&'static str>,
) -> View {
    if !open.get() {
        return rsx! { <></> };
    }

    let custom = class.unwrap_or("");

    rsx! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
            <div class={format!("w-full max-w-lg rounded-2xl bg-neutral-900 border border-neutral-800 shadow-2xl p-6 flex flex-col gap-6 {}", custom)}>
                <div class="flex items-center justify-between border-b border-neutral-800 pb-4">
                    <h2 class="text-lg font-bold text-white">{title}</h2>
                    <button
                        class="text-neutral-400 hover:text-white text-lg font-bold p-1 cursor-pointer"
                        onclick={move |_| open.set(false)}
                    >
                        {"✕"}
                    </button>
                </div>
            </div>
        </div>
    }
}

pub struct Toast;

impl Toast {
    pub fn success(msg: &str) {
        println!("✨ [Toast:Success] {}", msg);
    }

    pub fn error(msg: &str) {
        println!("❌ [Toast:Error] {}", msg);
    }
}
