use jot_math::macro_loop;

macro_loop! {
    @for feature in [
        game,
        gpu,
        input,
        math,
        window,
        renderer2d,
        fixed,
        scheme,
        collections,
        ecs,
        asset,
        scene,
        camera,
    ] {
        #[cfg(feature = @[@feature => str])]
        pub mod @feature {
            pub use @[jot_ @feature]::*;
        }
    }
}
