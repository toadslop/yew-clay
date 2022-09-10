use yew::Children;

pub enum FocusManagerChildren {
    Children(Children),
    Function(Box<dyn Fn() -> ()>),
}

impl PartialEq for FocusManagerChildren {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Children(l0), Self::Children(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => {
                let left: *const dyn Fn() -> () = l0.as_ref();
                let right: *const dyn Fn() -> () = r0.as_ref();
                left == right
            }
            _ => false,
        }
    }
}

impl core::fmt::Debug for FocusManagerChildren {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Children(arg0) => f.debug_tuple("Children").field(arg0).finish(),
            Self::Function(_) => f.debug_tuple("Function").field(&"dyn Fn() -> ()").finish(),
        }
    }
}
