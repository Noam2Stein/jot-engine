use jot_math::*;

mod macros;
mod render_shader_;
pub use render_shader_::*;

// Private

mod shader_type;

#[doc(hidden)]
pub mod _private_ {
    pub use jot_math::*;

    pub use super::shader_type::*;

    pub use const_format::concatcp;
    pub use const_random::const_random;
}
