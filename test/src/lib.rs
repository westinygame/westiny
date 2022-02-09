#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $delta:expr) => {
        assert!(($x - $y).abs() < $delta);
    };
}
