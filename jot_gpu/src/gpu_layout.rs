use super::*;

pub use crevice::{std140::AsStd140, std430::AsStd430};

pub type Std140<T> = <T as AsStd140>::Output;
pub type Std430<T> = <T as AsStd430>::Output;

pub use crevice;
