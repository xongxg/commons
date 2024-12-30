#[macro_export]
macro_rules! S {
    ($s:expr) => {
        String::from($s)
    };
}

#[test]
fn test_debug() {
    let world = "hello world";
    println!("{}", S!(world));
}
