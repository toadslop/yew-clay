use std::fmt::Debug;

use super::types::{AriaLabels, Years};
use chrono::{Month, NaiveDate};
use web_sys::MouseEvent;
use yew::{Callback, Component, Context, NodeRef, Properties};

#[derive(Debug, Properties, PartialEq)]
struct Props {
    aria_labels: AriaLabels,
    current_month: Month,

    #[prop_or_default]
    disabled: bool,
    months: Vec<&'static str>,
    on_dot_clicked: Callback<MouseEvent>,
    on_month_change: Callback<Month>,

    /// Path to the location of the spritemap resource.
    #[prop_or_default]
    spritemap: &'static str,

    years: Years,
}

struct DateNavigation {
    month_selector_ref: NodeRef,
    year_selector_ref: NodeRef,
}

impl DateNavigation {
    fn handle_change_month(ctx: &Context<Self>, month: Month) {
        let Props { current_month, .. } = ctx.props();
        //let new_month = *current_month + month;
        //let year = date.getFullYear();

        // if (memoizedYears.find((elem) => elem.value === year)) {
        // 	onMonthChange(date);
        // }
    }
}

impl Component for DateNavigation {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            month_selector_ref: NodeRef::default(),
            year_selector_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}
