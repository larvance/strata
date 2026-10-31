extern crate proc_macro;
use proc_macro::TokenStream;

/// Declares a Strata functional component with fine-grained reactivity.
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Passes through the function definition while annotating component metadata
    item
}

/// The declarative JSX/HTML-in-Rust templating macro.
#[proc_macro]
pub fn rsx(_input: TokenStream) -> TokenStream {
    // Future full recursive descent parser will lower tags into NodeBlueprint constructors.
    // For scaffolding, emit a placeholder View construction.
    "View { root: NodeBlueprint::Empty }".parse().unwrap()
}

/// Dynamic JavaScript object/array constructor macro.
#[proc_macro]
pub fn dynamic(_input: TokenStream) -> TokenStream {
    "JsValue::Null".parse().unwrap()
}
