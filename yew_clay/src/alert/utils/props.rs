use crate::button::ClayButtonGroup;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{virtual_dom::VChild, Callback, Children, Classes, NodeRef, Properties};
use yew_dom_attributes::props::html_element_props::HtmlElementProps;

use super::enums::{AlertDisplayType, AlertVariant, AutoCloseValue};

/// Props for ClayAlert. For details, check the docs:
/// <https://clayui.com/docs/components/alert/api.html#alert>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayAlertProps {
    /// Accepts a ClayButtonGroup in which to render buttons representing the possible actions
    /// that can be carried out on this alert.
    #[prop_or_default]
    pub actions: Option<VChild<ClayButtonGroup>>,

    /// Flag to indicate alert should automatically call `onClose`. It also
    /// accepts a duration (in ms) which indicates how long to wait. If `true`
    /// is passed in, the timeout will be 10000ms.
    #[prop_or_default]
    pub auto_close: Option<AutoCloseValue>,

    /// Callback function for when the 'x' icon is clicked.
    #[prop_or_default]
    pub on_close: Option<Callback<MouseEvent>>,

    /// Determines the style of the alert.
    #[prop_or_default]
    pub display_type: AlertDisplayType,

    /// Flag to indicate if close icon should be show. This prop is used in
    /// conjunction with the `onClose`prop;
    #[prop_or_default]
    pub hide_close_icon: Option<bool>,

    /// Path to the spritemap that Icon should use when referencing symbols.
    #[prop_or_default]
    pub spritemap: Option<String>,

    /// The summary of the Alert, often is something like 'Error' or 'Info'.
    #[prop_or_default]
    pub title: Option<String>,

    /// Determines the variant of the alert.
    #[prop_or_default]
    pub variant: Option<AlertVariant>,

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
