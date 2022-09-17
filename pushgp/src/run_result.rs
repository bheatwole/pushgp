use std::fmt::Debug;

/// This trait is a alias to avoid typing all the restrictions everytime we need to reference them
pub trait RunResult: Clone + Debug + PartialEq + 'static {}
