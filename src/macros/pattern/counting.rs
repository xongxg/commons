macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*}
}

#[test]
fn test_replace_expr() {
    assert_eq!(count_tts!(0 1 2), 3);

    let len = count_tts!(0 1 2 3 4);
    println!("len is {}", len);
}

macro_rules! count_tts1 {
    () => {
        0usize
    };
    ($head:tt $($tail:tt)*) => {1usize + count_tts1!($($tail)*)};
}

#[test]
fn test_count_tts1() {
    let len = count_tts1!(0 1 2 3 4);
    println!("len is {}", len);
}

macro_rules! count_tts2 {
    ($($tts:tt)*) => {
        <[()]>::len(&[$(replace_expr!($tts ())),*])
    };
}

#[test]
fn test_count_tts2() {
    let len = count_tts2!(0 1 2 3 4);
    println!("len is {}", len);
}
