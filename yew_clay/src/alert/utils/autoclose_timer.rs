use gloo_timers::callback::Timeout;
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
    timer: Option<Timeout>,
    time_to_close: &Option<u32>,
    started_time: &Option<f64>,
) -> Option<u32> {
    if timer.is_some() && time_to_close.is_some() {
        timer.unwrap().cancel();
        let time_to_close = time_to_close.unwrap();
        let started_time = started_time.unwrap_or(0.0);
        let time_to_close = (time_to_close) - (js_sys::Date::now() - started_time) as u32;
        if time_to_close > 0 {
            return Some(time_to_close);
        }
    }
    None
}

pub fn start(
    time_to_close: &Option<u32>,
    ctx: &Context<ClayAlert>,
) -> Option<(Option<f64>, Option<Timeout>)> {
    if let Some(time_to_close) = time_to_close {
        let started_time = Some(js_sys::Date::now());
        let timer = {
            let link = ctx.link().clone();
            Some(Timeout::new(*time_to_close, move || {
                link.send_message(Msg::Close)
            }))
        };
        return Some((started_time, timer));
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
