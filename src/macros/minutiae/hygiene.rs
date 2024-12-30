#[macro_export]
macro_rules! helper {
    () => {
        ()
    };
}

macro_rules! helped {
    () => {
        $crate::helper!()
    };
}

#[test]
fn test_helped() {
    helped!();
}

pub mod inner {
    pub fn foo() {}

    #[macro_export]
    macro_rules! call_foo {
        () => {
            $crate::inner::foo()
        };
    }
}
