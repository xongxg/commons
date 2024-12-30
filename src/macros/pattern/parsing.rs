macro_rules! function_item_matcher {
    (
        $(#[$meta:meta])*
        //  ^~~~attributes~~~~^
        $vis:vis fn $name:ident ($($arg_name:ident : $arg_ty:ty),* $(,)?)
        //                          ^~~~~~~~~~~~~~~~argument list~~~~~~~~~~~~~~~^
        $(->$ret_ty:ty)?
        //      ^~~~return type~~~^
        { $($body:tt)*}
        //      ^~~~~body~~~~^
    )=>{
        $(#[$meta])*
        $vis fn $name ($($arg_name : $arg_ty),*) $(-> $ret_ty)? { $($body)*}
    };
}

macro_rules! method_item_matcher {
    // self
    (
        $(#[$meta:meta])*
        //  ^~~~attributes~~~~^
        $vis:vis fn $name:ident ($self:ident $(, $arg_name:ident : $arg_ty:ty)* $(,)?)
        //                          ^~~~~~~~~~~~~~~~~~~~~argument list~~~~~~~~~~~~~~~~~~~~~~^
        $(->$ret_ty:ty)?
        {$($tt:tt)*}
    ) => {
        $(#[$meta])*
        $vis fn $name ($self $(, $arg_name : $arg_ty)*) $(-> $ret_ty)? { $($tt)* }
    }

    // mut self
    (
        $( #[$meta:meta] )*
        $vis:vis fn $name:ident ( mut $self:ident $(, $arg_name:ident : $arg_ty:ty )* $(,)? )
            $( -> $ret_ty:ty )?
            { $($tt:tt)* }
    ) => {
        $( #[$meta] )*
        $vis fn $name ( mut $self $(, $arg_name : $arg_ty )* ) $( -> $ret_ty )? { $($tt)* }
    };

    // &self
    (
        $( #[$meta:meta] )*
        $vis:vis fn $name:ident ( & $self:ident $(, $arg_name:ident : $arg_ty:ty )* $(,)? )
            $( -> $ret_ty:ty )?
            { $($tt:tt)* }
    ) => {
        $( #[$meta] )*
        $vis fn $name ( & $self $(, $arg_name : $arg_ty )* ) $( -> $ret_ty )? { $($tt)* }
    };

    // &mut self
    (
        $( #[$meta:meta] )*
        $vis:vis fn $name:ident ( &mut $self:ident $(, $arg_name:ident : $arg_ty:ty )* $(,)? )
            $( -> $ret_ty:ty )?
            { $($tt:tt)* }
    ) => {
        $( #[$meta] )*
        $vis fn $name ( &mut $self $(, $arg_name : $arg_ty )* ) $( -> $ret_ty )? { $($tt)* }
    }
}

macro_rules! struct_item_matcher {
    // unit-struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$meta])*
        $vis struct $name;
    };

    // Tuple-Struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(
            $(
             $(#[$field_meta:meta])*
            // ^~~~field attributes~~~~^
             $field_vis:vis $field_ty:ty
            // ^~~~~~a single field~~~~~~^
            ),*
            $(,)?
        );
    )=>{
        $(#[$meta])*
        $vis struct $name(
          $(
           $(#[$field_meta])*
           $field_vis $field_ty
          ),*
        );
    };

    // named-struct
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident{
            $(
             $(#[$field_meta:meta])*
             $field_vis:vis $field_name:ident : $field_ty:ty
            ),*
          $(,)?
        };
    )=>{
        $(#[$meta])*
        $vis struct $name{
           f$(
              $(#[$field_meta])*
              $field_vis $field_name: $field_ty
            ),*
        };
    }
}
