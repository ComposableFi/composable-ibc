#[macro_export]
macro_rules! log {
    ($self:expr, $($tt:tt)*) => {
        $self.log(&format!($($tt)*))
    };
}
