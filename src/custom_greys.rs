use ratatui::style::Color;

pub trait CustomGreys {
	fn select_grey() -> Self;
	fn ui_grey() -> Self;
}

impl CustomGreys for Color {
	fn select_grey() -> Self {
		Self::Indexed(242)
	}
	
	fn ui_grey() -> Self {
		Self::Indexed(238)
	}
}
