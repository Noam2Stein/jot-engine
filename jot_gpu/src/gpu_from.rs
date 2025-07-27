use super::*;

pub trait GpuFrom<T> {
    fn gpu_from(value: T, gpu: &Gpu) -> Self;
}

pub trait GpuInto<T> {
    fn gpu_into(self, gpu: &Gpu) -> T;
}

impl<T, U> GpuInto<U> for T
where
    U: GpuFrom<T>,
{
    fn gpu_into(self, gpu: &Gpu) -> U {
        U::gpu_from(self, gpu)
    }
}
