use crate::BYTES_PER_LINE;

#[derive(Clone, Copy)]
pub struct WindowSize {
	pub rows: usize,
	pub covered_rows: usize,
}

impl WindowSize {
	pub const fn visible_byte_count(&self) -> usize {
		self.hex_rows() * BYTES_PER_LINE
	}
	
	pub const fn hex_rows(&self) -> usize {
		self.rows - self.covered_rows
	}
}
