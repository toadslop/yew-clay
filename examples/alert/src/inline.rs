use crate::{CONTAINER_CLASS, SPRITEMAP};
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use yew::{html, html_nested, Component, Context, Html};
use yew::{Callback, NodeRef, Properties};
use yew_clay::AlertVariant;
use yew_clay::ClayButton;
use yew_clay::ClayButtonGroup;
use yew_clay::ClayContainer;
use yew_clay::{AlertDisplayType, ClayAlert};

pub struct InlineAlert {}

pub enum Msg {
    Dummy,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub node_ref: NodeRef,
}

impl Component for InlineAlert {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut props = ButtonProps::new();

        let cb: Callback<MouseEvent> = _ctx.link().callback(|_ev| Msg::Dummy);
        let event = Click::new(String::from("my-click"), cb);
        props.add_event(Box::new(event));

        let cb: Callback<web_sys::Event> = _ctx.link().callback(|_ev| Msg::Dummy);
        let event = Abort::new(String::from("my-abort"), cb);
        props.add_event(Box::new(event));

        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Inline"}</h2>
                <ClayAlert
                    display_type={AlertDisplayType::Info}
                    spritemap={SPRITEMAP}
                    title={"With a Button"}
                    variant={AlertVariant::Inline}
                    actions={{html_nested! {
                        <ClayButtonGroup>
                            <ClayButton alert={true}>{"View"}</ClayButton>
                        </ClayButtonGroup>
                    }}}>
                    {"This is an alert with a button!"}
                </ClayAlert>
            </ClayContainer>
        }
    }
}

use std::fmt::Debug;

pub trait ButtonEvent: Event {}

pub trait Event {
    /// Returns a string representing the key of a DOM attribute.
    fn get_key(&self) -> &str;

    /// Returns an `Option<String>` representing the value of a DOM
    /// attribute. `None` indicates a boolean attribute, such as `disabled`,
    /// which has no value.
    fn get_callback(&mut self) -> Box<dyn Fn(&web_sys::Event)>;

    fn get_id(&self) -> String;
}

struct Click {
    id: String,
    func: Option<Box<dyn Fn(&web_sys::Event)>>,
}

impl Click {
    const KEY: &'static str = "click";

    pub fn new(id: String, cb: Callback<MouseEvent>) -> Self {
        let func = move |e: &web_sys::Event| {
            let event = e.clone();
            cb.emit(event.dyn_into::<MouseEvent>().unwrap());
        };
        Self {
            id,
            func: Some(Box::new(func)),
        }
    }
}

impl Event for Click {
    fn get_key(&self) -> &str {
        Self::KEY
    }

    fn get_callback(&mut self) -> Box<(dyn Fn(&web_sys::Event))> {
        self.func.take().unwrap()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ButtonEvent for Click {}

struct Abort {
    id: String,
    func: Option<Box<dyn Fn(&web_sys::Event)>>,
}

impl Abort {
    const KEY: &'static str = "abort";

    pub fn new(id: String, cb: Callback<web_sys::Event>) -> Self {
        let func = move |e: &web_sys::Event| {
            let event = e.clone();
            cb.emit(event.dyn_into::<web_sys::Event>().unwrap());
        };
        Self {
            id,
            func: Some(Box::new(func)),
        }
    }
}

impl Event for Abort {
    fn get_key(&self) -> &str {
        Self::KEY
    }

    fn get_callback(&mut self) -> Box<(dyn Fn(&web_sys::Event))> {
        self.func.take().unwrap()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl ButtonEvent for Abort {}

pub struct ButtonProps {
    attributes: Vec<(ProcessAction, String, Option<String>)>,
    listeners: Vec<(ProcessAction, Box<dyn ButtonEvent>)>,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            listeners: Vec::new(),
        }
    }
    pub fn add_event(&mut self, event: Box<dyn ButtonEvent>) {
        let action = ProcessAction::Add;
        self.listeners.push((action, event))
    }

    pub fn inject(&mut self, elem: &Element) {
        for (action, mut event) in self.listeners.drain(..) {
            match action {
                ProcessAction::Add => {
                    let event_type = event.get_id();
                    let cb = event.get_callback();
                    let listener = EventListener::new(&elem, event_type, cb);
                }
                ProcessAction::Remove => todo!(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessAction {
    Add,
    Remove,
}
