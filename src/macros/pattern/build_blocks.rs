macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

macro_rules! as_pat {
    ($p:pat) => {
        $p
    };
}

macro_rules! as_stmt {
    ($s:stmt) => {
        $s
    };
}

macro_rules! as_ty {
    ($t:ty) => {
        $t
    };
}

#[test]
fn test_build_block() {
    as_item! {struct Dummy;};

    as_stmt! {{
        let as_pat!(_): as_ty!(_) = as_expr!(42);
    }};
}
