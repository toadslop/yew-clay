use self::context::ClayCardContext;
use yew::{Callback, Component, Context};

pub mod aspect_ratio;
mod body;
mod context;

pub const SPAN: &'static str = "span";
pub const DIV: &'static str = "div";

trait Interactive: Component {
    fn get_interactive(ctx: &Context<Self>) -> bool {
        if let Some((context, _)) = ctx.link().context::<ClayCardContext>(Callback::noop()) {
            context.interactive
        } else {
            false
        }
    }

    fn get_tag_name(interactive: bool) -> &'static str {
        if interactive {
            SPAN
        } else {
            DIV
        }
    }
}
