/// This is a trait for things that require a name at compile time.
pub trait StaticName {
    fn static_name() -> &'static str;
}
