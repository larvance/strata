//! Strata Codegen JS: Source-to-source transpiler emitting pure JavaScript & DOM code.

use strata_dom::NodeBlueprint;

pub struct JsTranspiler;

impl JsTranspiler {
    /// Transpiles an RSX NodeBlueprint into pure vanilla JavaScript DOM calls.
    pub fn emit_js(blueprint: &NodeBlueprint) -> String {
        let mut output = String::new();
        Self::render_node(blueprint, &mut output);
        output
    }

    fn render_node(blueprint: &NodeBlueprint, out: &mut String) {
        match blueprint {
            NodeBlueprint::Element { tag, classes, children } => {
                out.push_str(&format!(
                    "(() => {{ const el = document.createElement('{:?}'); ",
                    tag
                ));
                if !classes.is_empty() {
                    out.push_str(&format!("el.className = '{}'; ", classes.join(" ")));
                }
                for child in children {
                    out.push_str("el.appendChild(");
                    Self::render_node(child, out);
                    out.push_str("); ");
                }
                out.push_str("return el; })()");
            }
            NodeBlueprint::Text(txt) => {
                out.push_str(&format!("document.createTextNode('{}')", txt));
            }
            NodeBlueprint::Fragment(children) => {
                out.push_str("(() => { const frag = document.createDocumentFragment(); ");
                for child in children {
                    out.push_str("frag.appendChild(");
                    Self::render_node(child, out);
                    out.push_str("); ");
                }
                out.push_str("return frag; })()");
            }
            _ => out.push_str("document.createComment('')"),
        }
    }
}
