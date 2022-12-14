use super::utils::autoclose_timer::{
    close, get_time_to_close, initialize_autoclose_callbacks, pause, start,
};
use super::utils::element_generators::{
    gen_default_alert, gen_default_footer_element, gen_dismiss_button, gen_inline_footer_element,
    gen_stripe_alert, gen_title_element,
};
use super::{AlertDisplayType, AlertVariant, ClayAlertProps};
use crate::alert::utils::sub_components::ConditionalContainer;
use crate::layout::{ClayContentCol, ClayContentRow, ClayContentSection};
use gloo_events::EventListener;
use std::collections::HashMap;
use web_sys::MouseEvent;
use yew::{classes, html, Callback, Component, Context, Html, NodeRef};
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayAlert. For more info about ClayAlert, check the documentation:
/// <https://clayui.com/docs/components/alert.html>
pub struct ClayAlert {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, EventListener>,
    timer_id: Option<i32>,
    pause_timer: Option<Callback<MouseEvent>>,
    start_timer: Option<Callback<MouseEvent>>,
    started_time: Option<f64>,
    time_to_close: Option<u32>,
}

pub enum Msg {
    Close,
    PauseTimer,
    StartTimer,
}

impl ClayAlert {
    const ALERT: &'static str = "alert-";

    fn get_show_dismissible(
        on_close: &Option<Callback<MouseEvent>>,
        hide_close_icon: bool,
    ) -> bool {
        on_close.is_some() && !hide_close_icon
    }

    fn get_dismissible_class(show_dismissible: bool) -> Option<String> {
        match show_dismissible {
            true => Some("alert-dismissible".into()),
            false => None,
        }
    }

    fn get_variant_class(variant: &Option<AlertVariant>) -> Option<String> {
        if let Some(variant) = variant {
            match variant {
                AlertVariant::Feedback => Some("alert-feedback alert-indicator-start".into()),
                AlertVariant::Stripe => Some("alert-fluid".into()),
                AlertVariant::Inline => Some("alert-inline".into()),
            }
        } else {
            None
        }
    }

    fn get_display_class(display_type: &AlertDisplayType) -> Option<String> {
        let display_type_str = display_type.as_ref();
        let mut display_class = String::with_capacity(display_type_str.len() + Self::ALERT.len());
        display_class.push_str(Self::ALERT);
        display_class.push_str(display_type_str);
        Some(display_class)
    }
}

impl Component for ClayAlert {
    type Message = Msg;
    type Properties = ClayAlertProps;

    fn create(ctx: &Context<Self>) -> Self {
        let ClayAlertProps { auto_close, .. } = ctx.props().clone();
        let time_to_close = get_time_to_close(auto_close);

        let (pause_timer, start_timer) = initialize_autoclose_callbacks(ctx, time_to_close);

        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
            timer_id: None,
            start_timer,
            pause_timer,
            started_time: None,
            time_to_close,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                close(ctx);
                true
            }
            Msg::PauseTimer => {
                self.time_to_close = pause(
                    self.timer_id.take(),
                    &self.time_to_close,
                    &self.started_time,
                );
                false
            }
            Msg::StartTimer => {
                let maybe_start = start(&self.time_to_close, ctx);
                if let Some((started_time, handler)) = maybe_start {
                    self.started_time = started_time;
                    self.timer_id = handler;
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayAlertProps {
            class,
            children,
            variant,
            hide_close_icon,
            on_close,
            display_type,
            spritemap,
            title,
            actions,
            ..
        } = ctx.props().clone();

        let show_dismissible = Self::get_show_dismissible(&on_close, hide_close_icon);
        let dismissible_class = Self::get_dismissible_class(show_dismissible);
        let variant_class = Self::get_variant_class(&variant);
        let display_class = Self::get_display_class(&display_type);

        let start_timer = self.start_timer.clone();
        let pause_timer = self.pause_timer.clone();

        let stripe_alert_indicator = gen_stripe_alert(spritemap, &display_type, &variant);
        let default_alert_indicator = gen_default_alert(spritemap, &display_type, &variant);
        let title_element = gen_title_element(&title);
        let default_footer_element = gen_default_footer_element(&variant, &actions);
        let inline_footer_element = gen_inline_footer_element(&variant, &actions);
        let dismiss_button = gen_dismiss_button(show_dismissible, on_close, spritemap);

        html! {
            <div
                class={classes!(class, "alert", dismissible_class, variant_class, display_class)}
                role={"alert"}
                ref={self.node_ref.clone()}
                onmouseout={start_timer}
                onmouseover={pause_timer} >
                <ConditionalContainer {variant}>
                    <ClayContentRow class={"alert-autofit-row"}>
                        {stripe_alert_indicator}
                        <ClayContentCol expand={true}>
                            <ClayContentSection>
                                {default_alert_indicator}
                                {title_element}
                                {children}
                                {default_footer_element}
                            </ClayContentSection>
                        </ClayContentCol>
                        {inline_footer_element}
                    </ClayContentRow>
                    {dismiss_button}
                </ConditionalContainer>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let ClayAlertProps {
                auto_close,
                on_close,
                ..
            } = ctx.props().clone();

            if auto_close.is_some() && on_close.is_some() {
                self.start_timer
                    .as_ref()
                    .and_then(|cb| Some(cb.emit(MouseEvent::new("mouseover").unwrap())));
            }
        }

        if let Some(html_props) = &ctx.props().html_props {
            let html_props = html_props.clone();
            html_props.inject(&self.node_ref, &mut self.listeners);
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.pause_timer
            .as_ref()
            .and_then(|cb| Some(cb.emit(MouseEvent::new("mouseout").unwrap())));
    }
}
