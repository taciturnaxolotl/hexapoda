use ratatui::style::Color;

pub trait SelectGrey {
	fn select_grey() -> Self;
}

impl SelectGrey for Color {
	fn select_grey() -> Self {
		Self::Indexed(242)
	}
}
