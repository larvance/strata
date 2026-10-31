//! High-Level Button Components for Strata UI.

#![allow(non_snake_case)]

use strata_dom::{NodeBlueprint, View};
use strata_macro::{component, rsx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    loading: Option<bool>,
    disabled: Option<bool>,
    class: Option<&'static str>,
) -> View {
    let var = variant.unwrap_or_default();
    let sz = size.unwrap_or_default();

    let base = "inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 active:scale-95 disabled:opacity-50 disabled:pointer-events-none select-none";

    let var_classes = match var {
        ButtonVariant::Primary => "bg-indigo-600 hover:bg-indigo-500 text-white shadow-md shadow-indigo-600/20",
        ButtonVariant::Secondary => "bg-neutral-800 hover:bg-neutral-700 text-neutral-100",
        ButtonVariant::Outline => "border border-neutral-800 hover:border-neutral-700 text-white bg-transparent",
        ButtonVariant::Ghost => "text-neutral-400 hover:text-white hover:bg-neutral-800/60",
        ButtonVariant::Destructive => "bg-rose-600 hover:bg-rose-500 text-white shadow-md shadow-rose-600/20",
    };

    let sz_classes = match sz {
        ButtonSize::Sm => "h-8 px-3 text-xs gap-1.5",
        ButtonSize::Md => "h-10 px-4 text-sm gap-2",
        ButtonSize::Lg => "h-12 px-6 text-base gap-2.5",
    };

    let custom = class.unwrap_or("");
    let combined = format!("{base} {var_classes} {sz_classes} {custom}");

    rsx! {
        <button class={combined} disabled={disabled.unwrap_or(false)}>
            {if loading.unwrap_or(false) {
                rsx! { <span class="animate-spin mr-2">{"◌"}</span> }
            } else {
                rsx! { <></> }
            }}
        </button>
    }
}
