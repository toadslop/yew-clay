use strum::AsRefStr;

#[derive(AsRefStr, Debug, PartialEq, Clone)]
pub enum AutoCloseValue {
    Boolean(bool),
    Number(u32),
}

#[derive(AsRefStr, Debug, PartialEq, Clone, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AlertDisplayType {
    Danger,
    #[default]
    Info,
    Success,
    Warning,
}

#[derive(AsRefStr, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum AlertVariant {
    Feedback,
    Stripe,
    Inline,
}
