use std::time::Duration;

use gpui::{Div, ParentElement, Styled, div};

pub struct ComboState {
    pub(crate) prev_action: u16
}

pub struct Action {
    pub(crate) action: u16,
    pub(crate) meter_state: crate::game::meter_state::MeterState,
    pub(crate) combo_state: ComboState,
    pub(crate) encounter_time: Duration,
}

impl Action {
    pub fn render(&self) -> Div {
        div()
            .flex()
            .gap_2()
            .child(
                div()
                    .font_family("DejaVu Sans Mono")
                    .text_xl()
                    .child(
                        format!(
                            "{:02}:{:02}.{:03}", 
                            self.encounter_time.as_secs() % 60,
                            self.encounter_time.as_secs() / 60,
                            self.encounter_time.subsec_millis()
                        )
                    )
            )
            .child(self.meter_state.render())
    }
}