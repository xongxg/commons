use std::ops::Index;

macro_rules! vec_strs {
    (
        $(
        $e:expr
        )
        ,
        *
    ) => {{
        let mut v = Vec::new();
        $(
         v.push(format!("{}",$e));
        )*

        v
    }};
}

#[test]
fn test_vec_strs() {
    let s = vec_strs![1, "a", true, 3.14159f32];
    assert_eq!(s, &["1", "a", "true", "3.14159"]);
}

macro_rules! repeat_two {
    ($($i:ident)*, $($i2:ident)*) => {
        $( let $i: (); let $i2: (); )*
    }
}

// #[test]
// fn test_repeat_two() {
//     // println!("repeat is {}", repeat_two!(a b c d e f, u v w x y z));
//     repeat_two!(a b c d e f, u v w x y z);
//     std::num::Wrapping::<T>;
// }
