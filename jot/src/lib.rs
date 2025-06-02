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
    graphics,
    input,
    math,
    window,
}
