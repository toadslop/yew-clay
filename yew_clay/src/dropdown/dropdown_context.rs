use std::{fmt::Debug, rc::Rc};

#[derive(Default)]
struct DropDownContext {
    close: Option<Rc<dyn FnOnce() -> ()>>,
    close_on_click: bool,
}

impl Clone for DropDownContext {
    fn clone(&self) -> Self {
        Self {
            close: self.close.as_ref().map(|func| func.clone()),
            close_on_click: self.close_on_click,
        }
    }
}

impl Debug for DropDownContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DropDownContext")
            .field("close", &"Option<Rc<dyn FnOnce() -> ()>>")
            .field("closeOnClick", &self.close_on_click)
            .finish()
    }
}

impl PartialEq for DropDownContext {
    fn eq(&self, other: &Self) -> bool {
        let funcs_eq = if let (Some(this), Some(other)) = (&self.close, &other.close) {
            Rc::ptr_eq(&this, &other)
        } else {
            false
        };
        funcs_eq && self.close_on_click == other.close_on_click
    }
}
