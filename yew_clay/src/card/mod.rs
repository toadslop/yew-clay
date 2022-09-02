use self::context::ClayCardContext;
use yew::{Callback, Component, Context};

pub mod aspect_ratio;
mod body;
mod caption;
mod card_horizontal;
mod card_navigation;
mod context;

trait Interactive: Component {
    const SPAN: &'static str = "span";
    const DIV: &'static str = "div";

    fn get_interactive(ctx: &Context<Self>) -> bool {
        if let Some((context, _)) = ctx.link().context::<ClayCardContext>(Callback::noop()) {
            context.interactive
        } else {
            false
        }
    }

    fn get_tag_name(interactive: bool) -> &'static str {
        if interactive {
            Self::SPAN
        } else {
            Self::DIV
        }
    }
}
