use std::rc::Rc;

use yew::{Children, Classes, NodeRef, Properties};
use yew_dom_attributes::props::html_element_props::HtmlElementProps;

mod sheet;
pub use sheet::*;

mod sheet_footer;
pub use sheet_footer::*;

mod sheet_header;
pub use sheet_header::*;

mod sheet_section;
pub use sheet_section::*;

/// A generic set of props for container elements.
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ContainerProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// Intercepts the class tag from the parent so it can be modified with classes
    /// necessary for this container.
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
