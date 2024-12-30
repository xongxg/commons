#![feature(macro_metavar_expr)]
// macro_rules! foo {
//     () => {
//         macro_rules! bar {
//             ( $$( $$any:tt )* ) => { $$( $$any )* };
//         }
//     };
// }
//
// #[test]
// fn foo_macro() {
//     foo!();
// }

macro_rules! foo {
    ($($outer:ident($($inner:ident),*);)*) => {{
           // println!("count(outer, 0): $outer repeats {} times", ${count($outer)});

    }};
}
