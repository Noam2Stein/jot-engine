#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FixedTime<const FPS: u32> {
    accumulator: f64,
}

impl<const FPS: u32> FixedTime<FPS> {
    pub fn new() -> Self {
        Self { accumulator: 0.0 }
    }

    pub fn update<Flow: PartialEq + Default>(
        &mut self,
        delta_time: f64,
        mut fixed_update: impl FnMut() -> Flow,
    ) -> Flow {
        let time_step: f64 = 1.0 / FPS as f64;

        self.accumulator += delta_time;

        while self.accumulator >= 0.0 {
            self.accumulator -= time_step;

            let flow = fixed_update();

            if flow != Flow::default() {
                return flow;
            }
        }

        Flow::default()
    }
}
