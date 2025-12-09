use std::time::Duration;

use egui::{Color32, Id};
use egui_dnd::DragDropItem;
use uuid::Uuid;

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
struct SenState {
    getsu: bool,
    ka: bool,
    setsu: bool
}

struct SenRender {
    getsu_color: Color32,
    getsu_text: String,
    ka_color: Color32,
    ka_text: String,
    setsu_color: Color32,
    setsu_text: String,
}

impl SenState {
    fn get(&self) -> SenRender {
        SenRender {
            getsu_color: if self.getsu { Color32::from_rgb(0x58, 0x58, 0xff) } else { Color32::from_rgb(0xd0, 0xd0, 0xf7) },
            getsu_text: if self.getsu { "\u{23fa}".to_owned() } else { "\u{2b55}".to_owned() },
            ka_color: if self.ka { Color32::from_rgb(0xff, 0x58, 0x58) } else { Color32::from_rgb(0xf7, 0xd0, 0xd0) },
            ka_text: if self.ka { "\u{23fa}".to_owned() } else { "\u{2b55}".to_owned() },
            setsu_color: if self.setsu { Color32::from_rgb(0x58, 0xff, 0x58) } else { Color32::from_rgb(0xd0, 0xf7, 0xd0) },
            setsu_text: if self.setsu { "\u{23fa}".to_owned() } else { "\u{2b55}".to_owned() }
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq)]
pub struct MeterState {
    pub kenki: i8,
    sen: SenState,
    fugestu_timer: f32,
    fuka_timer: f32,
}

impl Default for MeterState {
    fn default() -> Self {
        MeterState {
            kenki: 0,
            sen: SenState { getsu: false, ka: false, setsu: false},
            fugestu_timer: 0f32,
            fuka_timer: 0f32,
        }
    }
}

impl MeterState {
    pub fn render(&self, ui: &mut egui::Ui) {
        let sen_render = self.sen.get();
        ui.horizontal(|ui| {
            ui.colored_label(sen_render.getsu_color, sen_render.getsu_text);
            ui.colored_label(sen_render.ka_color, sen_render.ka_text);
            ui.colored_label(sen_render.setsu_color, sen_render.setsu_text);

            ui.add(
                egui::ProgressBar::new((self.kenki as f32) / 100f32)
                    .fill(Color32::from_rgb(0xff, 0x80, 0x80))
                    .desired_width(100f32)
                    .corner_radius(0f32)
            )
        });
    }
}

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Action {
    pub meter_state: MeterState,
    pub timestamp: Duration,
    pub flag_delete: bool,
    pub id: Id
}

impl Default for Action {
    fn default() -> Self {
        Action {
            id: Id::new(Uuid::new_v4()),
            meter_state: MeterState::default(),
            timestamp: Duration::default(),
            flag_delete: false
        }
    }
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

impl DragDropItem for &mut Action {
    fn id(&self) -> Id {
        self.id
    }
}