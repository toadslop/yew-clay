use strum::Display;

#[derive(Display, Debug, PartialEq, Clone)]
pub enum AutoCloseValue {
    Boolean(bool),
    Number(u64),
}

#[derive(Display, Debug, PartialEq, Clone, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AlertDisplayType {
    Danger,
    #[default]
    Info,
    Success,
    Warning,
}

#[derive(Display, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum AlertVariant {
    Feedback,
    Stripe,
    Inline,
}
