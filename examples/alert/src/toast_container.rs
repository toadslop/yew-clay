use crate::{CONTAINER_CLASS, SPRITEMAP};
use yew::{html, Component, Context, Html};
use yew_clay::ClayContainer;
use yew_clay::ClayToastContainer;
use yew_clay::{AutoCloseValue, ClayAlert};

pub struct WithToastContainer {
    toast_items: Vec<f64>,
}

pub enum Msg {
    AddItem,
    RemoveItem(usize),
}

impl Component for WithToastContainer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            toast_items: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddItem => {
                self.toast_items.push(js_sys::Math::random());
                true
            }
            Msg::RemoveItem(index) => {
                self.toast_items.remove(index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Using with ToastContainer"}</h2>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::AddItem)}>
                        {"Add Alert"}
                    </button>
                </div>
                <ClayToastContainer>
                    {self.toast_items.iter().enumerate().map(|(index, value)| {html!{
                        <ClayAlert
                            auto_close={AutoCloseValue::Number(5000)}
                            key={index}
                            on_close={ctx.link().callback(move |_| Msg::RemoveItem(index))}
                            spritemap={SPRITEMAP}
                            title={"Hola:"}
                            >
                            {format!("My value is {}", value)}
                        </ClayAlert>
                    }}).collect::<Html>()}
                </ClayToastContainer>
            </ClayContainer>
        }
    }
}
