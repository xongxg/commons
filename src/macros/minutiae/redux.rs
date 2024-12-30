macro_rules! ambiguity {
    ($($i:ident)*; $i2:ident) => {
        // println!("$i {}", $i);
    };
}

#[test]
fn test_ambiguity() {
    ambiguity!(an_identifier; an_identifier2);
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a:ident+$b:ident) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

#[test]
fn test_capture_then_match_tokens() {
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );

    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!($m);};
}

macro_rules! what_is {
    (#[no_mangle]) => {
        "no_mangle attribute"
    };
    (#[inline]) => {
        "inline attribute"
    };
    ($($tts:tt)*)=>{
        concat!("Something else (", stringify!($($tts)*), ")")
    };
}

#[test]
fn test_what_is() {
    println!(
        "{}\n{}\n{}\n{}",
        what_is!(#[no_mangle]),
        what_is!(#[inline]),
        capture_then_what_is!(#[no_mangle]),
        capture_then_what_is!(#[inline]),
    );
}
