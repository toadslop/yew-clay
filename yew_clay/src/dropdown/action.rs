use yew::{classes, html, Children, Classes, Component, Html, NodeRef, Properties};
use yew_dom_attributes::button_props::ButtonProps;

use crate::ClayButton;

#[derive(Debug, Properties, PartialEq, Clone)]
struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    class: Classes,

    #[prop_or_default]
    other_props: Option<ButtonProps>,

    #[prop_or_default]
    node_ref: NodeRef,
}

struct ClayDropDownAction;

impl ClayDropDownAction {
    const DROPDOWN_SECTION: &'static str = "dropdown-section";
}

impl Component for ClayDropDownAction {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props {
            class,
            children,
            node_ref,
            other_props,
        } = ctx.props().clone();
        html! {
            <div class={classes!(class, Self::DROPDOWN_SECTION)}>
                <ClayButton block={true} node_ref={node_ref} button_props={other_props}>
                    {children}
                </ClayButton>
            </div>
        }
    }
}
