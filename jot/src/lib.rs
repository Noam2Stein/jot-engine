pub use jot_macros::*;

repetitive! {
    @for feature in [
        // core
        'window,
        'math,
        'gpu,
        'input,
        'collections,

        // basic
        'game,
        'asset,
        'scene,
        'fixed,
        'serialize,
        'scheme,

        // common
        'ecs,
        'camera,

        // 2d
        'renderer2d,
        'tilemap,
        'chunk,
    ] {
        #[cfg(feature = @str[feature])]
        pub mod @feature {
            pub use @['jot_ feature]::*;
        }
    }
}
