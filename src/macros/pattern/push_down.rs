// macro_rules! init_array {
//     [$e:expr; $n:tt] => {{
//         let e = $e;
//         accum!([$n,e.clone()]->[])
//     }}
// }
//
// macro_rules! accum {
//     ([3,$e:expr]->[$($body:tt)*]) => {accum!([2,$e]->[$($body)* $e,]);};
//     ([2,$e:expr]->[$($body:tt)*])=>{accum!([1,$e]->[$($body)* $e,]);};
//     ([1, $e:expr] -> [$($body:tt)*]) => { accum!([0, $e] -> [$($body)* $e,]) };
//     ([0, $_:expr] -> [$($body:tt)*]) => { [$($body)*] };
// }
//
// #[test]
// fn test_init() {
//     let strings: [String; 3] = init_array!["hello".to_string(); 3];
//     println!("{:?}", strings);
// }

macro_rules! init_array1 {
    [$e:expr; $n:tt] => {
        {
            let e = $e;
            [accum!($n,e.clone())]
        }
    };
}

macro_rules! accum1 {
    (3,$e:expr)=>{$e,accum1!(2,$e)};
    (2,$e:expr)=>{$e,accum1!(1,$e)};
    (1,$e:expr)=>{$e};
}

// #[test]
// fn test_init1() {
//     let strings: [String; 1] = init_array1!["hello1".to_string(); 1];
//     println!("{:?}", strings);
// }
