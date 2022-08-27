use std::collections::HashMap;

use crate::{CONTAINER_CLASS, SPRITEMAP};
use yew::{html, Component, Context, Html};
use yew_clay::ClayContainer;
use yew_clay::ClayToastContainer;
use yew_clay::{AutoCloseValue, ClayAlert};

pub struct WithToastContainer {
    toast_items: HashMap<usize, f64>,
    next_idx: usize,
}

pub enum Msg {
    AddItem(usize),
    RemoveItem(usize),
}

impl Component for WithToastContainer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            toast_items: HashMap::new(),
            next_idx: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddItem(idx) => {
                self.toast_items.insert(idx, js_sys::Math::random());
                self.next_idx = self.next_idx + 1;
                true
            }
            Msg::RemoveItem(index) => {
                self.toast_items.remove(&index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let next_idx = self.next_idx;
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Using with ToastContainer"}</h2>
                <div>
                    <button onclick={ctx.link().callback(move |_| Msg::AddItem(next_idx))}>
                        {"Add Alert"}
                    </button>
                </div>
                <ClayToastContainer>
                    {self.toast_items.iter().map(|(index, value)| {
                        let idx = *index;
                        html!{
                        <ClayAlert
                            auto_close={AutoCloseValue::Number(5000)}
                            key={idx}
                            on_close={ctx.link().callback(move |_| {

                                Msg::RemoveItem(idx)})}
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
