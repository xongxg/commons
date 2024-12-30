macro_rules! mixed_rules{
    ()=>{};
    (trace $name:ident; $($tail:tt)*)=>{{
           println!(concat!(stringify!($name),"={:?}"),$name);
        mixed_rules!($($tail)*);
    }};

    (trace $name:ident = $init:expr; $($tail:tt)*)=>{{
        let $name = $init;
        println!(concat!(stringify!($name),"={:?}"),$name);
        mixed_rules!($($tail)*);
    }};
}

#[test]
fn test_mixed_rules() {
    let a = 42;
    let b = "Ho-dee-oh-di-oh-di-oh!";
    let c = (false, 2, 'c');

    mixed_rules!(
        trace a;
        trace b;
        trace c;
        trace d = "They took her where they put the crazies.";
        trace a;
    )
}
