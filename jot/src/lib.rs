macro_rules! modules {
    ($($mod:ident), * $(,)?) => {$(
        paste::paste! {
            #[cfg(feature = "" $mod)]
            pub mod $mod {
                pub use [<jot_ $mod>]::*;
            }
        }
    )*};
}

modules! {
    game,
    gpu,
    input,
    math,
    window,
    renderer2d,
    fixed,
    scheme,
    collections,
}
