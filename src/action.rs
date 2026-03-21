use std::{cmp::min, collections::hash_set::Entry, convert::identity, fs::File, io::Write, iter, mem::{replace, swap}};
use ratatui::{style::Stylize, text::Span};

use crate::{BYTES_OF_PADDING, BYTES_PER_LINE, LINES_OF_PADDING, app::WindowSize, buffer::{Buffer, Mode, PartialAction}, cursor::Cursor, edit_action::EditAction};

#[derive(Clone, Copy)]
pub enum Action {
	QuitIfSaved,
	Quit,
	
	NormalMode,
	SelectMode,
	
	Goto,
	View,
	Replace,
	Space,
	
	MoveByteUp,
	MoveByteDown,
	MoveByteLeft,
	MoveByteRight,
	
	ExtendByteUp,
	ExtendByteDown,
	ExtendByteLeft,
	ExtendByteRight,
	
	GotoLineStart,
	GotoLineEnd,
	GotoFileStart,
	GotoFileEnd,
	
	ScrollDown,
	ScrollUp,
	
	PageCursorHalfDown,
	PageCursorHalfUp,
	
	PageDown,
	PageUp,
	
	MoveNextWordStart,
	MoveNextWordEnd,
	MovePreviousWordStart,
	
	ExtendNextWordStart,
	ExtendNextWordEnd,
	ExtendPreviousWordStart,
	
	CollapseSelection,
	FlipSelections,
	
	ExtendLineBelow,
	ExtendLineAbove,
	
	Delete,
	
	Undo,
	Redo,
	
	Save,
	
	PreviousBuffer,
	NextBuffer,
	
	CopySelectionOnNextLine,
	
	RotateSelectionsBackward,
	RotateSelectionsForward,
	
	KeepPrimarySelection,
	RemovePrimarySelection,
	
	SplitSelectionsInto1s,
	SplitSelectionsInto2s,
	SplitSelectionsInto3s,
	SplitSelectionsInto4s,
	SplitSelectionsInto5s,
	SplitSelectionsInto6s,
	SplitSelectionsInto7s,
	SplitSelectionsInto8s,
	SplitSelectionsInto9s,
	
	JumpToSelectedOffset,
	
	ToggleMark,
	
	AlignViewCenter,
	AlignViewBottom,
	AlignViewTop,
}

// actions that act on the app as a whole, not just one buffer
pub enum AppAction {
	QuitIfSaved,
	Quit,
	
	PreviousBuffer,
	NextBuffer,
}

impl Buffer {
	pub fn execute(&mut self, action: Action, window_size: WindowSize) -> Option<AppAction> {
		match action {
			Action::QuitIfSaved => return Some(AppAction::QuitIfSaved),
			Action::Quit => return Some(AppAction::Quit),
			
			Action::NormalMode => self.normal_mode(),
			Action::SelectMode => self.select_mode(),
			
			Action::Goto => self.goto(),
			Action::View => self.view(),
			Action::Replace => self.replace(),
			Action::Space => self.space(),
			
			Action::MoveByteUp => self.move_byte_up(window_size),
			Action::MoveByteDown => self.move_byte_down(window_size),
			Action::MoveByteLeft => self.move_byte_left(window_size),
			Action::MoveByteRight => self.move_byte_right(window_size),
			
			Action::ExtendByteUp => self.extend_byte_up(window_size),
			Action::ExtendByteDown => self.extend_byte_down(window_size),
			Action::ExtendByteLeft => self.extend_byte_left(window_size),
			Action::ExtendByteRight => self.extend_byte_right(window_size),
			
			Action::GotoLineStart => self.goto_line_start(),
			Action::GotoLineEnd => self.goto_line_end(),
			Action::GotoFileStart => self.goto_file_start(window_size),
			Action::GotoFileEnd => self.goto_file_end(window_size),
			
			Action::ScrollDown => self.scroll_down(window_size),
			Action::ScrollUp => self.scroll_up(window_size),
			
			Action::PageCursorHalfDown => self.page_cursor_half_down(window_size),
			Action::PageCursorHalfUp => self.page_cursor_half_up(window_size),
			
			Action::PageDown => self.page_down(window_size),
			Action::PageUp => self.page_up(window_size),
			
			Action::MoveNextWordStart => self.move_next_word_start(window_size),
			Action::MoveNextWordEnd => self.move_next_word_end(window_size),
			Action::MovePreviousWordStart => self.move_previous_word_start(window_size),
			
			Action::ExtendNextWordStart => self.extend_next_word_start(window_size),
			Action::ExtendNextWordEnd => self.extend_next_word_end(window_size),
			Action::ExtendPreviousWordStart => self.extend_previous_word_start(window_size),
			
			Action::CollapseSelection => self.collapse_selection(),
			Action::FlipSelections => self.flip_selection(),
			
			Action::ExtendLineBelow => self.extend_line_below(window_size),
			Action::ExtendLineAbove => self.extend_line_above(window_size),
			
			Action::Delete => self.delete(window_size),
			
			Action::Undo => self.undo(window_size),
			Action::Redo => self.redo(window_size),
			
			Action::Save => self.save(),
			
			Action::PreviousBuffer => return Some(AppAction::PreviousBuffer),
			Action::NextBuffer => return Some(AppAction::NextBuffer),
			
			Action::CopySelectionOnNextLine => self.copy_selection_on_next_line(),
			
			Action::RotateSelectionsBackward => self.rotate_selections_backward(),
			Action::RotateSelectionsForward => self.rotate_selections_forward(),
			
			Action::KeepPrimarySelection => self.keep_primary_selection(),
			Action::RemovePrimarySelection => self.remove_primary_selection(),
			
			Action::SplitSelectionsInto1s => self.split_selections_into_size(1),
			Action::SplitSelectionsInto2s => self.split_selections_into_size(2),
			Action::SplitSelectionsInto3s => self.split_selections_into_size(3),
			Action::SplitSelectionsInto4s => self.split_selections_into_size(4),
			Action::SplitSelectionsInto5s => self.split_selections_into_size(5),
			Action::SplitSelectionsInto6s => self.split_selections_into_size(6),
			Action::SplitSelectionsInto7s => self.split_selections_into_size(7),
			Action::SplitSelectionsInto8s => self.split_selections_into_size(8),
			Action::SplitSelectionsInto9s => self.split_selections_into_size(9),
			
			Action::JumpToSelectedOffset => self.jump_to_selected_offset(window_size),
			
			Action::ToggleMark => self.toggle_mark(),
			
			Action::AlignViewCenter => self.align_view_center(window_size),
			Action::AlignViewBottom => self.align_view_bottom(window_size),
			Action::AlignViewTop => self.align_view_top(),
		}
		
		None
	}
	
	const fn normal_mode(&mut self) {
		self.mode = Mode::Normal;
	}
	
	const fn select_mode(&mut self) {
		self.mode = Mode::Select;
	}
	
	const fn goto(&mut self) {
		self.partial_action = Some(PartialAction::Goto);
	}
	
	const fn view(&mut self) {
		self.partial_action = Some(PartialAction::View);
	}
	
	const fn replace(&mut self) {
		if !self.contents.is_empty() {
			self.partial_action = Some(PartialAction::Replace);
		}
	}
	
	const fn space(&mut self) {
		self.partial_action = Some(PartialAction::Space);
	}
	
	fn change_all_cursors(&mut self, transform: impl Fn(&mut Cursor)) {
		transform(&mut self.primary_cursor);
		
		for cursor in &mut self.cursors {
			transform(cursor);
		}
		self.cursors.sort_by_key(|cursor| cursor.head);
		
		self.combine_cursors_if_overlapping();
	}
	
	fn move_byte_up(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::move_byte_up);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn move_byte_down(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.move_byte_down(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn move_byte_left(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::move_byte_left);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn move_byte_right(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.move_byte_right(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_byte_up(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::extend_byte_up);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_byte_down(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_byte_down(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_byte_left(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::extend_byte_left);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_byte_right(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_byte_right(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn goto_line_start(&mut self) {
		self.change_all_cursors(Cursor::goto_line_start);
	}
	
	fn goto_line_end(&mut self) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.goto_line_end(max_contents_index));
	}
	
	fn goto_file_start(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::goto_file_start);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn goto_file_end(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.goto_file_end(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	pub fn scroll_down(&mut self, window_size: WindowSize) {
		if self.contents.len() <= BYTES_OF_PADDING { return; }
		
		self.scroll_position = min(
			self.scroll_position + BYTES_PER_LINE,
			self.contents.len() - BYTES_OF_PADDING - self.contents.len() % BYTES_PER_LINE
		);
		
		if window_size.hex_rows() > LINES_OF_PADDING * 2 {
			self.primary_cursor.clamp(
				self.scroll_position + BYTES_OF_PADDING,
				window_size.visible_byte_count() - BYTES_OF_PADDING * 2
			);
		} else {
			self.primary_cursor.clamp(self.scroll_position, window_size.visible_byte_count());
		}
		self.combine_cursors_if_overlapping();
	}
	
	pub fn scroll_up(&mut self, window_size: WindowSize) {
		self.scroll_position = self.scroll_position.saturating_sub(BYTES_PER_LINE);
		if window_size.hex_rows() > LINES_OF_PADDING * 2 {
			self.primary_cursor.clamp(
				self.scroll_position + BYTES_OF_PADDING,
				window_size.visible_byte_count() - BYTES_OF_PADDING * 2
			);
		} else {
			self.primary_cursor.clamp(self.scroll_position, window_size.visible_byte_count());
		}
		self.combine_cursors_if_overlapping();
	}
	
	fn page_cursor_half_down(&mut self, window_size: WindowSize) {
		if self.contents.len() <= BYTES_OF_PADDING { return; }
		
		let old_scroll_position = self.scroll_position;
		
		self.scroll_position = min(
			self.scroll_position + (window_size.visible_byte_count() / 2).next_multiple_of(BYTES_PER_LINE),
			self.contents.len() - BYTES_OF_PADDING - self.contents.len() % BYTES_PER_LINE
		);
		
		let scroll_position_change = self.scroll_position - old_scroll_position;
		let max_contents_index = self.max_contents_index();
		
		self.primary_cursor.head = min(
			self.primary_cursor.head + scroll_position_change,
			max_contents_index
		);
		self.primary_cursor.tail = min(
			self.primary_cursor.tail + scroll_position_change,
			max_contents_index
		);
		
		for cursor in &mut self.cursors {
			cursor.head = (cursor.head + scroll_position_change).min(max_contents_index);
			cursor.tail = (cursor.tail + scroll_position_change).min(max_contents_index);
		}
		
		self.combine_cursors_if_overlapping();
	}
	
	fn page_cursor_half_up(&mut self, window_size: WindowSize) {
		let old_scroll_position = self.scroll_position;
		
		self.scroll_position = self.scroll_position.saturating_sub(
			(window_size.visible_byte_count() / 2).next_multiple_of(BYTES_PER_LINE)
		);
		
		let scroll_position_change = old_scroll_position - self.scroll_position;
		let max_contents_index = self.max_contents_index();
		
		self.primary_cursor.head = min(
			self.primary_cursor.head - scroll_position_change,
			max_contents_index
		);
		self.primary_cursor.tail = min(
			self.primary_cursor.tail - scroll_position_change,
			max_contents_index
		);
		
		for cursor in &mut self.cursors {
			cursor.head = (cursor.head - scroll_position_change).min(max_contents_index);
			cursor.tail = (cursor.tail - scroll_position_change).min(max_contents_index);
		}
		
		self.combine_cursors_if_overlapping();
	}
	
	fn page_down(&mut self, window_size: WindowSize) {
		if self.contents.len() <= BYTES_OF_PADDING { return; }
		
		self.scroll_position = min(
			self.scroll_position + window_size.visible_byte_count(),
			self.contents.len() - BYTES_OF_PADDING - self.contents.len() % BYTES_PER_LINE
		);
		
		if window_size.hex_rows() > LINES_OF_PADDING * 2 {
			self.primary_cursor.clamp(
				self.scroll_position + BYTES_OF_PADDING,
				window_size.visible_byte_count() - BYTES_OF_PADDING * 2
			);
		} else {
			self.primary_cursor.clamp(self.scroll_position, window_size.visible_byte_count());
		}
		self.combine_cursors_if_overlapping();
	}
	
	fn page_up(&mut self, window_size: WindowSize) {
		self.scroll_position = self.scroll_position.saturating_sub(
			window_size.visible_byte_count()
		);
		
		if window_size.hex_rows() > LINES_OF_PADDING * 2 {
			self.primary_cursor.clamp(
				self.scroll_position + BYTES_OF_PADDING,
				window_size.visible_byte_count() - BYTES_OF_PADDING * 2
			);
		} else {
			self.primary_cursor.clamp(self.scroll_position, window_size.visible_byte_count());
		}
		self.combine_cursors_if_overlapping();
	}
	
	fn move_next_word_start(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.move_next_word_start(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn move_next_word_end(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.move_next_word_end(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn move_previous_word_start(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::move_previous_word_start);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_next_word_start(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_next_word_start(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_next_word_end(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_next_word_end(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_previous_word_start(&mut self, window_size: WindowSize) {
		self.change_all_cursors(Cursor::extend_previous_word_start);
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn collapse_selection(&mut self) {
		self.primary_cursor.collapse();
		
		for cursor in &mut self.cursors {
			cursor.collapse();
		}
	}
	
	fn flip_selection(&mut self) {
		self.primary_cursor.flip();
		
		for cursor in &mut self.cursors {
			cursor.flip();
		}
	}
	
	fn extend_line_below(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_line_below(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn extend_line_above(&mut self, window_size: WindowSize) {
		let max_contents_index = self.max_contents_index();
		self.change_all_cursors(|cursor| cursor.extend_line_above(max_contents_index));
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn delete(&mut self, window_size: WindowSize) {
		if !self.contents.is_empty() {
			self.execute_and_add(
				EditAction::Delete {
					primary_cursor: self.primary_cursor,
					cursors: self.cursors.clone(),
					primary_old_data: self.contents[self.primary_cursor.range()].into(),
					old_data: self.cursors
						.iter()
						.map(|cursor| self.contents[cursor.range()].to_vec())
						.collect(),
				},
				window_size
			);
		}
		
		if self.mode == Mode::Select {
			self.mode = Mode::Normal;
		}
	}
	
	fn undo(&mut self, window_size: WindowSize) {
		if self.time_traveling == Some(0) || self.edit_history.is_empty() { return; }
		
		let current_date = self.time_traveling
			.map_or(self.edit_history.len() - 1, |date| date - 1);
		
		self.time_traveling = Some(current_date);
		
		let edit_action = replace(
			&mut self.edit_history[current_date],
			EditAction::Placeholder
		);
		
		self.undo_edit(&edit_action, window_size);
		
		self.edit_history[current_date] = edit_action;
	}
	
	fn redo(&mut self, window_size: WindowSize) {
		let Some(previous_date) = self.time_traveling else { return; };
		
		let current_date = previous_date + 1;
		
		self.time_traveling = if current_date == self.edit_history.len() {
			None
		} else {
			Some(current_date)
		};
		
		let edit_action = replace(
			&mut self.edit_history[previous_date],
			EditAction::Placeholder
		);
		
		self.execute_edit(&edit_action, window_size);
		
		self.edit_history[previous_date] = edit_action;
	}
	
	fn save(&mut self) {
		let mut file = File::create(&self.file_path).unwrap();
		file.write_all(&self.contents).unwrap();
		
		self.last_saved_at = Some(
			self.time_traveling.unwrap_or(self.edit_history.len())
		);
	}
	
	fn copy_selection_on_next_line(&mut self) {
		let new_cursors: Vec<Cursor> = iter::once(&self.primary_cursor)
			.chain(&self.cursors)
			.filter_map(|cursor| {
				let number_of_lines_tall = (cursor.upper_bound() - cursor.lower_bound()) / BYTES_PER_LINE;
				let offset_to_add = (number_of_lines_tall + 1) * BYTES_PER_LINE;
				
				if cursor.lower_bound() + offset_to_add < self.contents.len() {
					Some(
						Cursor {
							head: min(cursor.head + offset_to_add, self.max_contents_index()),
							tail: min(cursor.tail + offset_to_add, self.max_contents_index())
						}
					)
				} else {
					None
				}
			})
			.collect();
		
		self.cursors.extend(new_cursors);
		self.cursors.sort_by_key(|cursor| cursor.head);
		
		self.combine_cursors_if_overlapping();
		
		self.rotate_selections_forward();
	}
	
	fn rotate_selections_backward(&mut self) {
		if self.cursors.is_empty() { return; }
		
		let next_cursor_index = self.cursors
			.binary_search_by_key(&self.primary_cursor.head, |cursor| cursor.head)
			.unwrap_or_else(identity);
		
		
		if next_cursor_index == 0 {
			let cursor_count = self.cursors.len();
			swap(&mut self.primary_cursor, &mut self.cursors[cursor_count - 1]);
			
			self.cursors.sort_by_key(|cursor| cursor.head);
		} else {
			swap(&mut self.primary_cursor, &mut self.cursors[next_cursor_index - 1]);
		}
	}
	
	fn rotate_selections_forward(&mut self) {
		if self.cursors.is_empty() { return; }
		
		let next_cursor_index = self.cursors
			.binary_search_by_key(&self.primary_cursor.head, |cursor| cursor.head)
			.unwrap_or_else(identity);
		
		if next_cursor_index == self.cursors.len() {
			swap(&mut self.primary_cursor, &mut self.cursors[0]);
			
			self.cursors.sort_by_key(|cursor| cursor.head);
		} else {
			swap(&mut self.primary_cursor, &mut self.cursors[next_cursor_index]);
		}
	}
	
	fn keep_primary_selection(&mut self) {
		self.cursors.clear();
	}
	
	fn remove_primary_selection(&mut self) {
		if self.cursors.is_empty() { return; }
		
		let next_cursor_index = self.cursors
			.binary_search_by_key(&self.primary_cursor.head, |cursor| cursor.head)
			.unwrap_or_else(identity);
		
		if next_cursor_index == self.cursors.len() {
			self.primary_cursor = self.cursors.remove(0);
		} else {
			self.primary_cursor = self.cursors.remove(next_cursor_index);
		}
	}
	
	fn split_selections_into_size(&mut self, size: usize) {
		if !iter::once(&self.primary_cursor)
			.chain(&self.cursors)
			.all(|cursor| cursor.len().is_multiple_of(size))
		{
			self.alert_message = Span::from(
				format!("not all selections are a multiple of {size} long")
			).red();
			return;
		}
		
		let mut new_cursors = iter::once(self.primary_cursor)
			.chain(self.cursors.iter().copied())
			.flat_map(|cursor| {
				cursor
					.range()
					.step_by(size)
					.map(|tail| Cursor { head: tail + size - 1, tail })
			});
		
		self.primary_cursor = new_cursors.next().unwrap();
		self.cursors = new_cursors.collect();
	}
	
	fn jump_to_selected_offset(&mut self, window_size: WindowSize) {
		if !iter::once(&self.primary_cursor)
			.chain(&self.cursors)
			.all(|cursor| {
				bytes_as_nat(&self.contents[cursor.range()])
					.is_some_and(|offset| offset < self.contents.len())
			})
		{
			if self.cursors.is_empty() {
				self.alert_message = Span::from(
					"selection is not a valid offset"
				).red();
			} else {
				self.alert_message = Span::from(
					"not all selections are valid offsets"
				).red();
			}
			return;
		}
		
		self.primary_cursor = Cursor::at(
			bytes_as_nat(&self.contents[self.primary_cursor.range()]).unwrap()
		);
		
		for cursor in &mut self.cursors {
			*cursor = Cursor::at(
				bytes_as_nat(&self.contents[cursor.range()]).unwrap()
			);
		}
		
		self.cursors.sort_by_key(|cursor| cursor.head);
		
		self.combine_cursors_if_overlapping();
		self.clamp_screen_to_primary_cursor(window_size);
	}
	
	fn toggle_mark(&mut self) {
		match self.marks.entry(self.primary_cursor.lower_bound()) {
			Entry::Occupied(occupied_entry) => { occupied_entry.remove(); },
			Entry::Vacant(vacant_entry) => vacant_entry.insert(),
		}
		
		for cursor in &self.cursors {
			match self.marks.entry(cursor.lower_bound()) {
				Entry::Occupied(occupied_entry) => { occupied_entry.remove(); },
				Entry::Vacant(vacant_entry) => vacant_entry.insert(),
			}
		}
	}
	
	const fn align_view_center(&mut self, window_size: WindowSize) {
		let half_a_screen = window_size.visible_byte_count() / 2;
		
		self.scroll_position = self.primary_cursor.head
			.saturating_sub(self.primary_cursor.head % BYTES_PER_LINE)
			.saturating_sub(half_a_screen - (half_a_screen % BYTES_PER_LINE));
	}
	
	fn align_view_bottom(&mut self, window_size: WindowSize) {
		self.scroll_position = self.primary_cursor.head
			.saturating_sub(self.primary_cursor.head % BYTES_PER_LINE)
			.saturating_sub(
				window_size
					.visible_byte_count()
					.saturating_sub(BYTES_PER_LINE + BYTES_OF_PADDING)
			)
			.min(self.max_contents_index() - self.max_contents_index() % BYTES_PER_LINE);
	}
	
	const fn align_view_top(&mut self) {
		self.scroll_position = self.primary_cursor.head
			.saturating_sub(self.primary_cursor.head % BYTES_PER_LINE)
			.saturating_sub(BYTES_OF_PADDING);
	}
}

// helpers
impl Buffer {
	pub fn clamp_screen_to_primary_cursor(&mut self, window_size: WindowSize) {
		if self.primary_cursor.head < self.scroll_position + BYTES_OF_PADDING {
			self.align_view_top();
		} else if self.primary_cursor.head > self.scroll_position + (window_size.visible_byte_count() - 1).saturating_sub(BYTES_OF_PADDING) {
			self.align_view_bottom(window_size);
		}
	}
}

fn bytes_as_nat(bytes: &[u8]) -> Option<usize> {
	bytes
		.iter()
		.rev() // little-endian
		.skip_while(|&&byte| byte == 0)
		.try_fold(usize::default(), |result, &byte| {
			Some(result.shl_exact(8)? | (byte as usize))
		})
}
