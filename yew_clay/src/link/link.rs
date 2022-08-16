use crate::link_context::LinkContext;
use gloo_events::EventListener;
use std::{collections::HashMap, marker::PhantomData, rc::Rc};
use web_sys::Element;
use yew::{
    classes, html, Callback, Children, Classes, Component, Context, Html, NodeRef, Properties,
};
use yew_dom_attributes::props::html_element_props::HtmlElementProps;
use yew_dom_attributes::props::DomInjector;

/// Yew implementation of ClayLink.
/// The type parameter T: LinkContext indicates which link context applies for this link.
/// (The link context allows customizing link tags throughout the app).
/// If you don't need a context, set T to DefaultLinkContext.
///
/// <https://clayui.com/docs/components/link.html>
pub struct ClayLink<T: LinkContext> {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
    phantom: PhantomData<T>,
}

impl<'a, T: LinkContext> ClayLink<T> {
    const BTN: &'a str = "btn";
    const BTN_BLOCK: &'a str = "btn-block";
    const BTN_MONOSPACED: &'a str = "btn-monospaced";
    const BTN_OUTLINE_BORDERLESS: &'a str = "btn-outline-borderless";
    const BTN_SMALL: &'a str = "btn-small";
    const BTN_PRIMARY: &'a str = "btn-primary";
    const BTN_SECONDARY: &'a str = "btn-secondary";
    const BTN_UNSTYLED: &'a str = "btn-unstyled";
    const BTN_OUTLINE_PRIMARY: &'a str = "btn-outline-primary";
    const BTN_OUTLINE_SECONDARY: &'a str = "btn-outline-secondary";
    const BTN_OUTLINE_UNSTYLED: &'a str = "btn-outline-unstyled";
    const LINK_MONOSPACED: &'a str = "link-monospaced";
    const LINK_OUTLINE: &'a str = "link-outline";
    const LINK_BORDERLESS: &'a str = "link-outline-borderless";
    const LINK_PRIMARY: &'a str = "link-primary";
    const LINK_SECONDARY: &'a str = "link-secondary";
    const LINK_UNSTYLED: &'a str = "link-unstyled";
    const LINK_OUTLINE_PRIMARY: &'a str = "link-outline-primary";
    const LINK_OUTLINE_SECONDARY: &'a str = "link-outline-secondary";
    const LINK_OUTLINE_UNSTYLED: &'a str = "link-outline-unstyled";

    fn get_btn_class(button: &LinkButton) -> Option<&'a str> {
        match button {
            LinkButton::Boolean(is_button) => match is_button {
                true => (),
                false => return None,
            },
            LinkButton::Options(_) => (),
        }
        Some(Self::BTN)
    }

    fn get_btn_block_class(button: &LinkButton, block: Option<bool>) -> Option<&'a str> {
        let block = block.unwrap_or(false);
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.block,
        };

        if button || block {
            Some(Self::BTN_BLOCK)
        } else {
            None
        }
    }

    fn get_btn_monospaced_class(button: &LinkButton, monospaced: bool) -> Option<&'a str> {
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.monospaced,
        };

        if button || monospaced {
            Some(Self::BTN_MONOSPACED)
        } else {
            None
        }
    }

    fn get_borderless_class(borderless: bool) -> Option<&'a str> {
        if borderless {
            Some(Self::BTN_OUTLINE_BORDERLESS)
        } else {
            None
        }
    }

    fn get_btn_small_class(button: &LinkButton, small: Option<bool>) -> Option<&'a str> {
        let small = small.unwrap_or(false);
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.small,
        };

        if button || small {
            Some(Self::BTN_SMALL)
        } else {
            None
        }
    }

    fn get_btn_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
        borderless: bool,
    ) -> Option<&'a str> {
        if let Some(display_type) = display_type {
            if !outline && !borderless {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::BTN_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::BTN_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::BTN_UNSTYLED),
                };
            }
        }
        None
    }

    fn get_btn_outline_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
        borderless: bool,
    ) -> Option<&'a str> {
        if let Some(display_type) = display_type {
            if outline || borderless {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::BTN_OUTLINE_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::BTN_OUTLINE_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::BTN_OUTLINE_UNSTYLED),
                };
            }
        };
        None
    }

    fn get_bool_class(boolean: bool, class: &'a str) -> Option<&'a str> {
        if boolean {
            Some(class)
        } else {
            None
        }
    }

    fn get_link_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
    ) -> Option<&'a str> {
        if !outline {
            if let Some(display_type) = display_type {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::LINK_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::LINK_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::LINK_UNSTYLED),
                };
            }
        }
        None
    }

    fn get_link_outline_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
    ) -> Option<&'a str> {
        if outline {
            if let Some(display_type) = display_type {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::LINK_OUTLINE_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::LINK_OUTLINE_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::LINK_OUTLINE_UNSTYLED),
                };
            }
        }
        None
    }
}

/// Props for ClayLink. For details, check the docs:
/// <https://clayui.com/docs/components/link/api.html>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayLinkProps {
    /// Renders the button as a block element.
    #[prop_or_default]
    pub block: Option<bool>,

    /// Flag to indicate if link should be borderless.
    #[prop_or(false)]
    pub borderless: bool,

    /// Config for button props
    #[prop_or_default]
    pub button: Option<LinkButton>,

    /// Determines how the link is displayed.
    #[prop_or_default]
    pub display_type: Option<LinkDisplayType>,

    /// Flag to indicate if link should be monospaced.
    #[prop_or(false)]
    pub monospaced: bool,

    /// Flag to indicate if link need have an outline.
    #[prop_or(false)]
    pub outline: bool,

    /// Indicates button should be a small variant.
    #[prop_or_default]
    pub small: Option<bool>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<Rc<HtmlElementProps>>,
}

pub enum Msg {}

impl<T: LinkContext + Clone + PartialEq + 'static> Component for ClayLink<T> {
    type Message = Msg;
    type Properties = ClayLinkProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
            phantom: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayLinkProps {
            class,
            children,
            node_ref,
            button,
            block,
            monospaced,
            borderless,
            small,
            outline,
            display_type,
            ..
        } = ctx.props().clone();
        let base_html = html! {<>{children}</>};

        let class = if let Some(button) = button {
            let btn_class = Self::get_btn_class(&button);
            let btn_block_class = Self::get_btn_block_class(&button, block);
            let btn_monospaced_class = Self::get_btn_monospaced_class(&button, monospaced);
            let borderless_class = Self::get_borderless_class(borderless);
            let btn_small_class = Self::get_btn_small_class(&button, small);
            let display_class = Self::get_btn_display_class(&display_type, outline, borderless);
            let outline_display_class =
                Self::get_btn_outline_display_class(&display_type, outline, borderless);

            classes!(
                class,
                btn_class,
                btn_block_class,
                btn_monospaced_class,
                borderless_class,
                btn_small_class,
                display_class,
                outline_display_class
            )
        } else {
            let link_monospaced_class = Self::get_bool_class(monospaced, Self::LINK_MONOSPACED);
            let link_outline_class = Self::get_bool_class(outline, Self::LINK_OUTLINE);
            let link_borderless = Self::get_bool_class(borderless, Self::LINK_BORDERLESS);
            let link_display_class = Self::get_link_display_class(&display_type, outline);
            let link_outline_display_class =
                Self::get_link_outline_display_class(&display_type, outline);

            classes!(
                class,
                link_monospaced_class,
                link_outline_class,
                link_borderless,
                link_display_class,
                link_outline_display_class
            )
        };

        if let Some((context, _)) = ctx.link().context::<T>(Callback::noop()) {
            context.render(base_html, class, node_ref)
        } else {
            html! {
              <a class={class} ref={node_ref}>
                {base_html}
              </a>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(custom_props) = &ctx.props().html_props {
            let mut custom_props = custom_props.clone();
            Rc::make_mut(&mut custom_props).inject(&self.node_ref, &mut self.listeners);
            custom_props
                .get_props_update_callback()
                .emit(custom_props.clone());
        }

        if let Some(elem) = &self.node_ref.cast::<Element>() {
            let target = elem.has_attribute("target");
            let rel = elem.has_attribute("rel");
            if target && !rel {
                match elem.set_attribute("rel", "noreferrer noopener") {
                    Ok(_) => (),
                    Err(err) => gloo_console::error!(err),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum LinkButton {
    Boolean(bool),
    Options(LinkButtonOptions),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkButtonOptions {
    pub block: bool,
    pub monospaced: bool,
    pub small: bool,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum LinkDisplayType {
    Primary,
    Secondary,
    Unstyled,
}
