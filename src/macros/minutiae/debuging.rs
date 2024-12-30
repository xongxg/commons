#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*)=>{each_tt!($($rest)*);};
}

#[test]
fn test_debug() {
    each_tt!(foo bar baz quux);
    // trace_macros!(true);
}

macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
}

#[test]
fn test_sing() {
    // sing! {
    //     ^ < @ < . @ *
    //     '\x08' '{' '"' _ # ' '
    //     - @ '$' && / _ %
    //     ! ( '\t' @ | = >
    //     ; '\x08' '\'' + '$' ? '\x7f'
    //     , # '"' ~ | ) '\x07'
    // }
}
