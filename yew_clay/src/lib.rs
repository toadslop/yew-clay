#[cfg(feature = "alert")]
mod alert;
#[cfg(feature = "alert")]
pub use alert::*;

#[cfg(feature = "badge")]
mod badge;
#[cfg(feature = "badge")]
pub use badge::*;

#[cfg(feature = "breadcrumb")]
mod breadcrumb;
#[cfg(feature = "breadcrumb")]
pub use breadcrumb::*;

#[cfg(feature = "button")]
mod button;
#[cfg(feature = "button")]
pub use button::*;

#[cfg(feature = "card")]
mod card;
#[cfg(feature = "card")]
pub use card::aspect_ratio::{ContainerAspectRatioType, Props};

#[cfg(feature = "icon")]
mod icon;
#[cfg(feature = "icon")]
pub use icon::*;

#[cfg(feature = "layout")]
mod layout;
#[cfg(feature = "layout")]
pub use layout::*;

#[cfg(feature = "link")]
mod link;
#[cfg(feature = "link")]
pub use link::*;

#[cfg(feature = "date-picker")]
mod date_picker;
#[cfg(feature = "date-picker")]
pub use date_picker::*;

trait HasBoolClass {
    fn get_bool_class(&self, boolean: Option<bool>, class_name: &'static str) -> Option<String> {
        if let Some(boolean) = boolean {
            self.match_bool(boolean, class_name)
        } else {
            None
        }
    }

    fn match_bool(&self, boolean: bool, class_name: &'static str) -> Option<String> {
        match boolean {
            true => Some(class_name.into()),
            false => None,
        }
    }
}
