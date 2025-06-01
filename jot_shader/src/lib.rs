mod macros;

pub trait ShaderType {}
pub trait ShaderFn<I: ShaderType, O: ShaderType> {}
