#[macro_export]
macro_rules! bitmask {
    ($first:expr $(, $bit:expr)* $(,)?) => {
        (1 << $first) $(| (1 << $bit))*
    };
}
