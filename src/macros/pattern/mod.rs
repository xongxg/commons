mod internal_rules;
mod push_down;
mod tt_bundle;
mod build_blocks;
mod counting;
mod tt_muncher;
mod parsing;

macro_rules! recognize_tree {
    (larch) => {
        println!("#1, the Larch.")
    };
    (redwood) => {
        println!("#2, the Mighty Redwood.")
    };
    (fir) => {
        println!("#3, the Fir.")
    };
    (chestnut) => {
        println!("#4, the Horse Chestnut.")
    };
    (pine) => {
        println!("#5, the Scots Pine.")
    };
    ($($other:tt)*) => {
        println!("I don't know; some kind of birch maybe?")
    };
}

macro_rules! expand_to_larch {
    () => {
        larch
    };
}

#[test]
fn test_expand_to_larch() {
    recognize_tree!(expand_to_larch!());
}

macro_rules! call_with_larch {
    ($callback:ident) => {
        $callback!(larch)
    };
}

macro_rules! callback {
    ($callback:ident($($args:tt)*)) => {
        // $callback!(larch)
        $callback!($($args)*);
    };
}

#[test]
fn test_call_with_larch() {
    call_with_larch!(recognize_tree);
    callback!(callback(println("Yes please!")));
}
