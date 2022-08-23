use std::rc::Rc;
use yew_dom_attributes::anchor_props::AnchorProps;

#[derive(Clone, Debug, PartialEq)]
pub struct LinkContext {
    pub tag: String,
    pub props: Rc<AnchorProps>,
}
