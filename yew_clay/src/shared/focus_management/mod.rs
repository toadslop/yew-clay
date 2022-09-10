use web_sys::Element;
use yew::NodeRef;

// Putting this on hold; seems it's for some react-specific details

fn is_fiber_host_component_focusable() -> bool {
    false
}

fn collect_focusable_elements(element: Element, focusable_elements: &mut Vec<Element>) {
    if let Some(child) = element.first_element_child() {
        collect_focusable_elements(child, focusable_elements)
    };

    if let Some(sibling) = element.next_element_sibling() {
        collect_focusable_elements(sibling, focusable_elements)
    };

    if is_fiber_host_component_focusable() {
        focusable_elements.push(element);
    }
}

fn get_focusable_elements_in_scope(fiber_node: NodeRef) -> Vec<Element> {
    let mut focusable_elements: Vec<Element> = Vec::new();
    let element = match fiber_node.cast::<Element>() {
        Some(elem) => elem,
        None => return focusable_elements,
    };

    if let Some(child) = element.first_element_child() {
        collect_focusable_elements(child, &mut focusable_elements)
    };

    focusable_elements
}

trait FocusManagement {
    fn get_next_focus_in_doc_ref() -> Option<NodeRef>;
    fn get_prev_focus_in_doc_ref() -> Option<NodeRef>;
    fn move_focus_in_scope(scope: NodeRef) {
        let fiber_focus_elements = get_focusable_elements_in_scope(scope);
    }
}
