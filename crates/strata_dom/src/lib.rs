//! Strata DOM: Fine-grained reactive signal graph and DOM representations.

use std::cell::RefCell;
use std::rc::Rc;

/// Represents an opaque handle to a rendered UI subtree.
#[derive(Debug, Clone)]
pub struct View {
    pub root: NodeBlueprint,
}

#[derive(Clone)]
pub enum NodeBlueprint {
    Element {
        tag: Tag,
        classes: Vec<String>,
        children: Vec<NodeBlueprint>,
    },
    Text(String),
    DynamicText(Rc<dyn Fn() -> String>),
    Fragment(Vec<NodeBlueprint>),
    Empty,
}

impl std::fmt::Debug for NodeBlueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element { tag, classes, children } => f
                .debug_struct("Element")
                .field("tag", tag)
                .field("classes", classes)
                .field("children", children)
                .finish(),
            Self::Text(t) => f.debug_tuple("Text").field(t).finish(),
            Self::DynamicText(_) => f.debug_tuple("DynamicText").field(&"<dynamic>").finish(),
            Self::Fragment(c) => f.debug_tuple("Fragment").field(c).finish(),
            Self::Empty => write!(f, "Empty"),
        }
    }
}

impl NodeBlueprint {
    pub fn element(tag: Tag, classes: Vec<String>, children: Vec<NodeBlueprint>) -> Self {
        Self::Element { tag, classes, children }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    pub fn fragment(children: Vec<NodeBlueprint>) -> Self {
        Self::Fragment(children)
    }
}

/// Supported HTML tag variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tag {
    Div,
    Span,
    Button,
    Input,
    Header,
    Nav,
    Main,
    Section,
    Aside,
    Footer,
    H1,
    H2,
    H3,
    H4,
    P,
    Ul,
    Ol,
    Li,
    A,
    Img,
    Svg,
    Path,
    Canvas,
    Custom(&'static str),
}

/// Fine-grained reactive state container.
#[derive(Clone)]
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
}

impl<T: Clone + 'static> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(initial)),
        }
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.value.borrow_mut())
    }
}

pub fn use_signal<T: Clone + 'static>(initial: T) -> Signal<T> {
    Signal::new(initial)
}

pub fn use_state<T: Clone + 'static>(init: impl FnOnce() -> T) -> (Signal<T>, Signal<T>) {
    let s = Signal::new(init());
    (s.clone(), s)
}

pub fn use_memo<T: Clone + 'static>(f: impl Fn() -> T + 'static) -> Signal<T> {
    Signal::new(f())
}

pub fn use_effect<F>(_f: F)
where
    F: Fn() -> Option<Box<dyn FnOnce()>> + 'static,
{
    // Reactive effect subscriber hook
}
