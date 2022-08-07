use strum::Display;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "icon")]
pub mod icon;

#[cfg(feature = "layout")]
pub mod layout;

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
