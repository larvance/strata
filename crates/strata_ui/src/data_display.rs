//! Data Display Components for Strata UI.

#![allow(non_snake_case)]

use strata_dom::{NodeBlueprint, View};
use strata_macro::{component, rsx};

#[component]
pub fn Card(class: Option<&'static str>) -> View {
    let custom = class.unwrap_or("");
    let base = "rounded-2xl bg-neutral-900/60 border border-neutral-800 shadow-xl overflow-hidden";
    let combined = format!("{base} {custom}");

    rsx! {
        <div class={combined}>
            <div class="p-6">{"Card Content"}</div>
        </div>
    }
}

#[component]
pub fn Stat(
    label: &'static str,
    value: &'static str,
    trend: Option<f32>,
    class: Option<&'static str>,
) -> View {
    let custom = class.unwrap_or("");

    rsx! {
        <div class={format!("p-5 rounded-2xl bg-neutral-900/60 border border-neutral-800 shadow-lg flex flex-col justify-between {}", custom)}>
            <div class="flex items-center justify-between text-neutral-400 text-xs font-mono uppercase">
                <span>{label}</span>
                {if let Some(t) = trend {
                    let trend_color = if t >= 0.0 { "text-emerald-400" } else { "text-rose-400" };
                    rsx! { <span class={format!("font-mono font-semibold {}", trend_color)}>{format!("{t:+.1}%")}</span> }
                } else {
                    rsx! { <></> }
                }}
            </div>
            <div class="mt-3 text-2xl font-bold font-mono text-white tracking-tight">
                {value}
            </div>
        </div>
    }
}

#[component]
pub fn Avatar(
    fallback: &'static str,
    class: Option<&'static str>,
) -> View {
    let custom = class.unwrap_or("");
    let base = "w-9 h-9 rounded-full bg-gradient-to-tr from-indigo-500 to-purple-600 flex items-center justify-center text-xs font-bold text-white ring-2 ring-neutral-800 select-none";
    let combined = format!("{base} {custom}");

    rsx! {
        <div class={combined}>
            {fallback}
        </div>
    }
}
