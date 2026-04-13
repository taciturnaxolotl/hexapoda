use ratatui::{style::{Color, Style}, text::Span};

pub fn render_address(address: usize) -> Span<'static> {
	Span {
		style: style_for_address(address),
		content: format!("{address:08x}").into()
	}
}

pub const fn style_for_address(address: usize) -> Style {
	if address.is_multiple_of(0x100) {
		Style::new().fg(Color::Rgb(0x68, 0x99, 0xA0))
	} else {
		Style::new().fg(Color::Rgb(0x8A, 0xBB, 0xC3))
	}
}
