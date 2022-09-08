/// The context helps to identify if the FocusScope is being declared nested, to
/// avoid focus being controlled by more than one focus manager, we stop event
/// propagation to prevent the parent focus generator from doing anything.
#[derive(Clone, Debug, PartialEq)]
pub struct FocusConflictContext(bool);
