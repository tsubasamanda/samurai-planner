use gpui::*;
use gpui_component::progress::Progress;

pub struct SenState {
	pub(crate) getsu: bool,
	pub(crate) ka: bool,
	pub(crate) setsu: bool
}

pub struct MeterState {
	pub(crate) sen: SenState,
	pub(crate) kenki: u8
}

impl MeterState {
	pub fn render(&self) -> Div {
		div()
			.flex()
			.child(
				div()
					.text_color(if self.sen.getsu { rgb(0x5858ff) } else { rgb(0xd0d0f7) })
					.child(if self.sen.getsu { "●" } else { "○" })
			)
			.child(
				div()
					.text_color(if self.sen.ka { rgb(0xff5858) } else { rgb(0xf7d0d0) })
					.child(if self.sen.ka { "●" } else { "○" })
			)
			.child(
				div()
					.text_color(if self.sen.setsu { rgb(0x58ff58) } else { rgb(0xd0f7d0) })
					.child(if self.sen.setsu { "●" } else { "○" })
			)
			.child(
				Progress::new()
					.value(self.kenki.into())
			)
	}
}