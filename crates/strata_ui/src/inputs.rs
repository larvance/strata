//! High-Level Form Input Components for Strata UI.

#![allow(non_snake_case)]

use strata_dom::{NodeBlueprint, Signal, View};
use strata_macro::{component, rsx};

#[component]
pub fn Input(
    label: Option<&'static str>,
    placeholder: Option<&'static str>,
    value: Option<Signal<String>>,
    disabled: Option<bool>,
    class: Option<&'static str>,
) -> View {
    let custom = class.unwrap_or("");
    let base = "w-full h-10 bg-neutral-900 border border-neutral-800 rounded-lg px-3.5 text-sm text-white placeholder-neutral-500 focus:outline-none focus:border-indigo-500 transition-colors";
    let combined = format!("{base} {custom}");

    rsx! {
        <div class="flex flex-col gap-1.5 w-full">
            {if let Some(lbl) = label {
                rsx! { <label class="text-xs font-mono uppercase tracking-wider text-neutral-400">{lbl}</label> }
            } else {
                rsx! { <></> }
            }}
            <input
                type="text"
                class={combined}
                placeholder={placeholder.unwrap_or("")}
                disabled={disabled.unwrap_or(false)}
            />
        </div>
    }
}

#[component]
pub fn Select(
    label: Option<&'static str>,
    options: Vec<&'static str>,
    value: Option<Signal<String>>,
    class: Option<&'static str>,
) -> View {
    let custom = class.unwrap_or("");
    let base = "w-full h-10 bg-neutral-900 border border-neutral-800 rounded-lg px-3 text-sm text-white focus:outline-none focus:border-indigo-500 transition-colors";
    let combined = format!("{base} {custom}");

    rsx! {
        <div class="flex flex-col gap-1.5 w-full">
            {if let Some(lbl) = label {
                rsx! { <label class="text-xs font-mono uppercase tracking-wider text-neutral-400">{lbl}</label> }
            } else {
                rsx! { <></> }
            }}
            <select class={combined}>
                {options.into_iter().map(|opt| rsx! { <option>{opt}</option> })}
            </select>
        </div>
    }
}

#[component]
pub fn Switch(
    label: Option<&'static str>,
    description: Option<&'static str>,
    checked: Signal<bool>,
) -> View {
    let is_active = checked.get();
    let bg_color = if is_active { "bg-indigo-600" } else { "bg-neutral-800" };
    let knob_pos = if is_active { "translate-x-5" } else { "translate-x-0" };

    rsx! {
        <div class="flex items-center justify-between w-full py-1">
            <div class="flex flex-col">
                {if let Some(lbl) = label {
                    rsx! { <span class="text-sm font-medium text-white">{lbl}</span> }
                } else {
                    rsx! { <></> }
                }}
                {if let Some(desc) = description {
                    rsx! { <span class="text-xs text-neutral-400">{desc}</span> }
                } else {
                    rsx! { <></> }
                }}
            </div>
            <button
                class={format!("w-11 h-6 rounded-full transition-colors relative p-0.5 cursor-pointer {}", bg_color)}
                onclick={move |_| checked.update(|c| *c = !*c)}
            >
                <div class={format!("w-5 h-5 rounded-full bg-white transition-transform duration-200 {}", knob_pos)} />
            </button>
        </div>
    }
}
