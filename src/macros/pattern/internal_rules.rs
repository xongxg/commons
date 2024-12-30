#[macro_export]
macro_rules! foo {
    (@as_expr $e:expr) => {$e};
    ($($tts:tt)*)=>{
        foo!(@as_expr $($tts)*)
    }
}

#[test]
fn test_internal_rules() {
    println!(foo!("as_expr bb"));
}
