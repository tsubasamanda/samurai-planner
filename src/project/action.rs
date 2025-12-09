use std::time::Duration;

struct SenState {
    getsu: bool,
    ka: bool,
    setsu: bool
}

pub struct MeterState {
    pub kenki: i8,
    sen: SenState
}

impl Default for MeterState {
    fn default() -> Self {
        MeterState { kenki: 0, sen: SenState { getsu: false, ka: false, setsu: false} }
    }
}

impl MeterState {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {

        });
    }
}

#[derive(Default)]
pub struct Action {
    meter_state: MeterState,
    timestamp: Duration
}

impl Action {
    pub fn timestamp(&self) -> String {
        format!(
            "{:02}:{:02}.{:03}", 
            self.timestamp.as_secs() % 60,
            self.timestamp.as_secs() / 60,
            self.timestamp.subsec_millis()
        )
    }
}