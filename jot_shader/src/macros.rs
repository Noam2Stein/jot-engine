#[macro_export]
macro_rules! shader_mod {
    { $vis:vis $ident:ident: $($tt:tt)* } => {
        $vis mod $ident {
            #[doc(hidden)]
            pub struct _ShaderModType_;

            $crate::_shader_mod_items_! { $($tt)* }
        }
    };
}

#[macro_export]
macro_rules! shader_interface {
    { $vis:vis $ident:ident: $($tt:tt)* } => {
        $vis mod $ident {
            #[doc(hidden)]
            pub unsafe trait _ShaderInterfaceTrait_ {}
        }
    };
}

// Private

#[doc(hidden)]
#[macro_export]
macro_rules! _shader_mod_items_ {
    () => {};

    (
        $vis:vis struct $ident:ident {}

        $($tt:tt)*
    ) => {
        $vis struct $ident {}

        $crate::_shader_mod_items_! { $($tt)* }
    };

        (
        $vis:vis fn $ident:ident() {}

        $($tt:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        $vis struct $ident;

        $crate::_shader_mod_items_! { $($tt)* }
    };
}
