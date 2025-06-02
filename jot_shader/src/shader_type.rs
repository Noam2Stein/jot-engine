use super::*;

pub trait ShaderType {
    const INFO: ShaderTypeInfo;
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderTypeInfo {
    Primitive(ShaderPrimitive),
    Vec {
        prim: ShaderPrimitive,
        len: ShaderLength,
    },
    Mat {
        prim: ShaderPrimitive,
        columns: ShaderLength,
        rows: ShaderLength,
    },
    Struct {
        id: &'static str,
        fields: &'static [ShaderField],
    },
}
#[derive(Debug, Clone, Copy)]
pub enum ShaderPrimitive {
    Bool,
    F32,
    I32,
    U32,
    S32,
}
#[derive(Debug, Clone, Copy)]
pub enum ShaderLength {
    Two,
    Three,
    Four,
}
#[derive(Debug, Clone, Copy)]
pub struct ShaderField {
    pub offset: usize,
    pub type_: ShaderTypeInfo,
}

impl ShaderType for f32 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Primitive(ShaderPrimitive::F32);
}
impl ShaderType for i32 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Primitive(ShaderPrimitive::I32);
}
impl ShaderType for u32 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Primitive(ShaderPrimitive::U32);
}
impl ShaderType for s32 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Primitive(ShaderPrimitive::S32);
}

impl ShaderType for FVec2 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::F32,
        len: ShaderLength::Two,
    };
}
impl ShaderType for IVec2 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::I32,
        len: ShaderLength::Two,
    };
}
impl ShaderType for UVec2 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::U32,
        len: ShaderLength::Two,
    };
}
impl ShaderType for SVec2 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::S32,
        len: ShaderLength::Two,
    };
}

impl ShaderType for FVec3 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::F32,
        len: ShaderLength::Three,
    };
}
impl ShaderType for IVec3 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::I32,
        len: ShaderLength::Three,
    };
}
impl ShaderType for UVec3 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::U32,
        len: ShaderLength::Three,
    };
}
impl ShaderType for SVec3 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::S32,
        len: ShaderLength::Three,
    };
}

impl ShaderType for FVec4 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::F32,
        len: ShaderLength::Four,
    };
}
impl ShaderType for IVec4 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::I32,
        len: ShaderLength::Four,
    };
}
impl ShaderType for UVec4 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::U32,
        len: ShaderLength::Four,
    };
}
impl ShaderType for SVec4 {
    const INFO: ShaderTypeInfo = ShaderTypeInfo::Vec {
        prim: ShaderPrimitive::S32,
        len: ShaderLength::Four,
    };
}
