use jot_scheme_proc_macros::InputType_Local;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, InputType_Local)]
pub struct Axis {
    pub positive: Value,
    pub negative: Value,
}

impl Axis {
    pub fn value(&self) -> i8 {
        self.positive.value as i8 - self.negative.value as i8
    }

    pub fn as_s32(&self) -> s32 {
        s32::from_i8(self.value()) / s32::int(16)
    }
}

pub trait IntoAxisBindingPair {
    fn into_axis_binding_pair(self) -> (Bindings<Value>, Bindings<Value>);
}

impl<P: Into<Bindings<Value>>, N: Into<Bindings<Value>>> IntoAxisBindingPair for (P, N) {
    fn into_axis_binding_pair(self) -> (Bindings<Value>, Bindings<Value>) {
        (self.0.into(), self.1.into())
    }
}
impl<T: IntoAxisBindingPair> IntoAxisBindingPair for Flat<T> {
    fn into_axis_binding_pair(self) -> (Bindings<Value>, Bindings<Value>) {
        let (positive, negative) = self.0.into_axis_binding_pair();

        (positive.flat(), negative.flat())
    }
}

repetitive! {
    @for len in 0..=16 {
        @let GenericParams = @{
            @for i in 0..len {
                @['T i]: IntoAxisBindingPair,
            }
        };
        @let GenericArgs = @{
            @for i in 0..len {
                @['T i],
            }
        };
        @let Tuple = @{(
            @for i in 0..len {
                @['T i],
            }
        )};

        impl<@GenericParams> From<@Tuple> for Bindings<Axis> {
            fn from(#[allow(unused_variables)] value: @Tuple) -> Self {
                #[allow(unused_mut)]
                let mut output = Self::default();

                @for i in 0..len {
                    let (positive, negative) = value.@i.into_axis_binding_pair();

                    output.positive += &positive;
                    output.negative += &negative;
                }

                output
            }
        }
    }
}
