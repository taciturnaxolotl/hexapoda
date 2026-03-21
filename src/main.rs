#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_possible_truncation)]
#![feature(get_disjoint_mut_helpers)]
#![feature(exact_bitshifts)]
#![feature(hash_set_entry)]

use app::App;
use crossterm::{QueueableCommand, event::{DisableMouseCapture, EnableMouseCapture}};

mod app;
mod buffer;
mod config;
mod cursor;
mod action;
mod edit_action;

mod cardinality;
mod empty_span;
mod custom_greys;

const BYTES_PER_LINE: usize = 0x10;
const BYTES_PER_CHUNK: usize = 4;
const CHUNKS_PER_LINE: usize = BYTES_PER_LINE / BYTES_PER_CHUNK;

const LINES_OF_PADDING: usize = 5;
const BYTES_OF_PADDING: usize = LINES_OF_PADDING * BYTES_PER_LINE;

// TODO:
// - zz/zt/zb
// - resizing can move the cursor off the screen
// - constant for 5 lines of padding
// - pad in clamp function, always
// - search
// - s/A-k/A-K
// - C-a/C-x
// - modifications
//   - insert/append
//     - mode
//     - add to edit history when *leaving* insert mode
//   - replace-and-keep-going
//     - mode
//   - change
// - edit character panel
//   - modifier on existing keys like teehee? or jump to panel?
//     - if jump to panel, space?
// - visual gg/G
// - jumplist
// - y/p
// - [/] to cycle view offset?
// - gj jump to entered offset
// - repeat X times (dec and hex)
// - jump relative to last marker?

// future directions
// - 'views' for bytes (i8/16/etc u8/16/etc 20.12/8.4/etc)
//   - how to fit??! `-128` longer than `80`
// - utf8?
// - diffing

// when AsciiChar is stabilized, use it instead of char everywhere

fn main() {
	let mut app = App::new();
	let mut terminal = ratatui::init();
	crossterm::terminal::enable_raw_mode().unwrap();
	terminal.backend_mut().queue(EnableMouseCapture).unwrap();
	
	while !app.should_quit {
		terminal.draw(|frame| {
			frame.render_widget(&app, frame.area());
		}).unwrap();
		
		app.handle_events(&mut terminal);
	}
	
	terminal.backend_mut().queue(DisableMouseCapture).unwrap();
	crossterm::terminal::disable_raw_mode().unwrap();
	ratatui::restore();
	
	// dbg!(app.edit_history);
	
	for log in app.buffers.iter().flat_map(|buffer| &buffer.logs) {
		println!("{log}");
	}
}
