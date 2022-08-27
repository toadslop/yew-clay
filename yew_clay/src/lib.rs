mod button;

#[cfg(feature = "button")]
pub use button::*;

mod icon;
#[cfg(feature = "icon")]
pub use icon::*;

mod layout;
#[cfg(feature = "layout")]
pub use layout::*;

mod alert;
#[cfg(feature = "alert")]
pub use alert::*;

mod badge;
#[cfg(feature = "badge")]
pub use badge::*;

mod breadcrumb;
#[cfg(feature = "breadcrumb")]
pub use breadcrumb::*;

mod link;
#[cfg(feature = "link")]
pub use link::*;

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

// TODO: Create a string pool for class names; use cfg feature any for any packages that need it
// should optimize size somewhat
