use gloo_timers::callback::Timeout;
use std::time::{Duration, Instant};
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
    time_to_close: &Option<Duration>,
    started_time: &Option<Instant>,
) -> Option<Duration> {
    if timer.is_some() && time_to_close.is_some() {
        timer.unwrap().cancel();
        let time_to_close = time_to_close.unwrap();
        let started_time = started_time.unwrap_or(Instant::now());
        let time_to_close = time_to_close - started_time.elapsed();
        if time_to_close.as_millis() > 0 {
            return Some(time_to_close);
        }
    }
    None
}

pub fn start(
    time_to_close: &Option<Duration>,
    ctx: &Context<ClayAlert>,
) -> Option<(Option<Instant>, Option<Timeout>)> {
    if let Some(time_to_close) = time_to_close {
        let started_time = Some(Instant::now());
        let timer = {
            let link = ctx.link().clone();
            Some(Timeout::new(
                time_to_close.as_millis().try_into().unwrap(),
                move || link.send_message(Msg::Close),
            ))
        };
        return Some((started_time, timer));
    }
    None
}

pub fn get_time_to_close(auto_close: Option<AutoCloseValue>) -> Option<Duration> {
    if let Some(auto_close) = auto_close {
        match auto_close {
            AutoCloseValue::Boolean(val) => match val {
                true => Some(Duration::new(10_000, 0)),
                false => None,
            },
            AutoCloseValue::Number(val) => Some(Duration::new(val, 0)),
        }
    } else {
        None
    }
}

pub fn initialize_autoclose_callbacks(
    ctx: &Context<ClayAlert>,
    time_to_close: Option<Duration>,
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
