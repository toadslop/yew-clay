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
