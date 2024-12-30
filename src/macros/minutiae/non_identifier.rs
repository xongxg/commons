use std::future::Ready;

macro_rules! what_is {
    (self) => {
        "the keyword `self`"
    };
    ($i:ident) => {
        concat!("the identifier `", stringify!($i), "`")
    };
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {
        $c!($i)
    };
}

#[test]
fn test_call_with_ident() {
    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
}

// macro_rules! make_self_mutable {
//     ($i:ident) => {
//         let mut $i = self;
//     };
// }
//
// #[derive(Debug)]
// struct Dummy(i32);
//
// impl Dummy {
//     fn double(i: i32) -> Self {
//         make_self_mutable!(mut_self);
//         mut_self.0 *= 2;
//         mut_self
//     }
// }

#[test]
fn test_make_self_mutable() {
    // println!("{}", make_self_mutable!(self));
    // println!("{:?}", Dummy::double(2));
}
