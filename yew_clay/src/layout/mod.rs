mod col;
pub use col::*;

mod container;
pub use container::*;

mod container_fluid;
pub use container_fluid::*;

mod content;
pub use content::*;

mod sheet;
pub use sheet::*;

mod row;
pub use row::*;
use strum::Display;

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
