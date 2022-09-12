use yew::Callback;

#[derive(Default, Clone, PartialEq)]
struct DropDownContext {
    close: Option<Callback<()>>,
    close_on_click: bool,
}
