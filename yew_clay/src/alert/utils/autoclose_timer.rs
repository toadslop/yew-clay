use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MouseEvent;
use yew::{Callback, Context};

use crate::alert::{AutoCloseValue, ClayAlert, Msg};

pub fn close(ctx: &Context<ClayAlert>) {
    ctx.props()
        .on_close
        .as_ref()
        .and_then(|cb| Some(cb.emit(MouseEvent::new("mousedown").unwrap())));
}

pub fn pause(
    time_id: Option<i32>,
    time_to_close: &Option<u32>,
    started_time: &Option<f64>,
) -> Option<u32> {
    if time_id.is_some() && time_to_close.is_some() {
        web_sys::window()
            .unwrap()
            .clear_timeout_with_handle(time_id.unwrap());

        let time_to_close = time_to_close.unwrap();
        let started_time = started_time.unwrap_or(0.0);
        let time_to_close = time_to_close
            .checked_sub((js_sys::Date::now() - started_time) as u32)
            .unwrap_or(0);

        if time_to_close > 0 {
            return Some(time_to_close);
        }
    }
    None
}

pub fn start(
    time_to_close: &Option<u32>,
    ctx: &Context<ClayAlert>,
) -> Option<(Option<f64>, Option<i32>)> {
    if let Some(time_to_close) = time_to_close {
        let started_time = Some(js_sys::Date::now());
        let link = ctx.link().clone();
        let callback = Closure::<dyn Fn()>::new(move || link.send_message(Msg::Close));

        let time_id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                *time_to_close as i32,
            )
            .unwrap();

        callback.forget();

        return Some((started_time, Some(time_id)));
    }
    None
}

pub fn get_time_to_close(auto_close: Option<AutoCloseValue>) -> Option<u32> {
    if let Some(auto_close) = auto_close {
        match auto_close {
            AutoCloseValue::Boolean(val) => match val {
                true => Some(10_000),
                false => None,
            },
            AutoCloseValue::Number(val) => Some(val),
        }
    } else {
        None
    }
}

pub fn initialize_autoclose_callbacks(
    ctx: &Context<ClayAlert>,
    time_to_close: Option<u32>,
) -> (Option<Callback<MouseEvent>>, Option<Callback<MouseEvent>>) {
    if time_to_close.is_some() {
        let pause_timer = ctx
            .link()
            .callback(move |_event: MouseEvent| Msg::PauseTimer);

        let start_timer = ctx
            .link()
            .callback(move |_event: MouseEvent| Msg::StartTimer);

        (Some(pause_timer), Some(start_timer))
    } else {
        (None, None)
    }
}
