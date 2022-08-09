use strum::Display;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "icon")]
pub mod icon;

#[cfg(feature = "layout")]
pub mod layout;

#[cfg(feature = "alert")]
pub mod alert;

#[cfg(feature = "badge")]
pub mod badge;

/// An enum specifying size varients.
#[derive(Debug, PartialEq, Clone, Display)]
pub enum Sizing {
    #[strum(serialize = "sm")]
    Small,
    #[strum(serialize = "md")]
    Medium,
    #[strum(serialize = "lg")]
    Large,
    #[strum(serialize = "xl")]
    XLarge,
}

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
