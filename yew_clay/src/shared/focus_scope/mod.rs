use self::focuse_manager_children::FocusManagerChildren;
use yew::{Callback, Component, FocusEvent, Properties};

mod focus_conflict_context;
mod focuse_manager_children;

#[derive(Debug, Properties, PartialEq)]
struct Props {
    /// Flag indicates whether the focus will also be controlled with the right
    /// and left arrow keys.
    #[prop_or_default]
    arrow_keys_left_right: bool,

    /// Flag that indicates if the focus will be controlled by the arrow keys.
    /// Disabling means that it will still be controlled by tab and shift + tab.
    #[prop_or_default]
    arrow_keys_up_down: bool,

    children: FocusManagerChildren,

    onfocus: Callback<FocusEvent>,
}

// TODO: Continue implementing focus scope:  https://github.com/liferay/clay/blob/master/packages/clay-shared/src/FocusScope.tsx
// Next: complete props (add onFocus) and continue

struct FocusScope;

impl Component for FocusScope {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}
