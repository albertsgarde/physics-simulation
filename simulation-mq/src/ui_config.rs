#[derive(Debug, Clone)]
pub struct UiConfig {
    ticks_per_second: f32,
}

impl UiConfig {
    pub fn new(ticks_per_second: f32) -> Self {
        Self { ticks_per_second }
    }

    pub fn ticks_per_second(&self) -> f32 {
        self.ticks_per_second
    }
}
