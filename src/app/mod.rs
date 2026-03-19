use std::{cmp::min, env, process::exit};
use crossterm::{event::{self, Event, KeyEvent}, terminal::window_size};
use crate::{BYTES_PER_LINE, buffer::Buffer, config::Config};

mod widget;

pub struct App {
	pub config: Config,
	
	pub buffers: Vec<Buffer>,
	pub current_buffer_index: usize,
	
	pub window_size: WindowSize,
	
	pub should_quit: bool,
	
	pub logs: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct WindowSize {
	pub rows: usize,
	pub covered_rows: usize,
}

impl App {
	pub fn new() -> Self {
		let buffers: Vec<Buffer> = env::args()
				.skip(1)
				.map(Into::into)
				.map(Buffer::new)
				.collect();
		
		if buffers.is_empty() {
			println!("please provide at least one file as input");
			exit(1);
		}
		
		Self {
			config: Config::default(),
			
			buffers,
			current_buffer_index: 0,
			
			window_size: WindowSize {
				rows: window_size().unwrap().rows as usize,
				// 1 because of the status line
				covered_rows: 1,
			},
			
			should_quit: false,
			
			logs: Vec::new(),
		}
	}
	
	#[allow(clippy::too_many_lines)]
	pub fn handle_events(&mut self) {
		#[allow(clippy::collapsible_match)]
		match event::read().unwrap() {
			Event::Resize(_, height) => {
				self.window_size.rows = height as usize;
			}
			Event::Key(key_event) => self.handle_key(key_event),
			// Event::Mouse(mouse_event) => {
			// 	mouse_event.kind
			// },
			_ => {}
		}
	}
	
	fn handle_key(&mut self, key_event: KeyEvent) {
		self.buffers[self.current_buffer_index]
			.handle_key(key_event, &self.config, self.window_size);
		
		if self.current_buffer().should_close {
			self.buffers.remove(self.current_buffer_index);
			
			if self.buffers.is_empty() {
				self.should_quit = true;
			} else {
				self.current_buffer_index = min(
					self.current_buffer_index,
					self.buffers.len() - 1
				);
			}
		}
	}
	
	fn current_buffer(&self) -> &Buffer {
		&self.buffers[self.current_buffer_index]
	}
}

impl WindowSize {
	pub const fn visible_byte_count(&self) -> usize {
		self.hex_rows() * BYTES_PER_LINE
	}
	
	pub const fn hex_rows(&self) -> usize {
		self.rows - self.covered_rows
	}
}
