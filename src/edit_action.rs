use std::{cmp::min, convert::identity, iter};
use crate::{buffer::Buffer, cursor::Cursor};

#[derive(Debug)]
pub enum EditAction {
	// this is a hacky workaround to allow disjoint borrows for
	// undoing/redoing edits. because the undo/redo operations need
	// to borrow an EditAction from edit_history, but should never touch
	// edit_history themselves, we can swap out the action in question,
	// then swap it back in once the undo/redo is done
	Placeholder,
	Delete {
		primary_cursor: Cursor,
		cursors: Vec<Cursor>,
		
		primary_old_data: Vec<u8>,
		old_data: Vec<Vec<u8>>
	},
	Replace {
		primary_cursor: Cursor,
		cursors: Vec<Cursor>,
		
		primary_old_data: Vec<u8>,
		old_data: Vec<Vec<u8>>,
		
		new_byte: u8
	},
	// Insert {
	// 	primary_cursor: Cursor,
	// 	cursors: Vec<Cursor>,
	//  which side of cursor? append/insert
	// 	new_data: Vec<u8>
	// }
}

impl Buffer {
	pub fn execute_and_add(&mut self, edit_action: EditAction) {
		assert!(!matches!(edit_action, EditAction::Placeholder));
		
		self.execute_edit(&edit_action);
		
		if let Some(date) = self.time_traveling {
			self.edit_history.truncate(date);
			self.time_traveling = None;
			
			if self.last_saved_at.is_some_and(|it| it > date) {
				self.last_saved_at = None;
			}
		}
		
		self.edit_history.push(edit_action);
	}
	
	pub fn execute_edit(&mut self, edit_action: &EditAction) {
		match edit_action {
			EditAction::Placeholder => unreachable!(),
			EditAction::Delete { primary_cursor, cursors, .. } => self.delete_at(*primary_cursor, cursors),
			EditAction::Replace {
				primary_cursor, cursors, primary_old_data: _, old_data: _, new_byte
			} => self.replace_at_with(*primary_cursor, cursors, *new_byte),
		}
	}
	
	pub fn undo_edit(&mut self, edit_action: &EditAction) {
		match edit_action {
			EditAction::Placeholder => unreachable!(),
			EditAction::Delete {
				primary_cursor, cursors, primary_old_data, old_data
			} => self.undo_delete_at(*primary_cursor, cursors, primary_old_data, old_data),
			EditAction::Replace {
				primary_cursor, cursors, primary_old_data, old_data, ..
			} => self.undo_replace_at_with(*primary_cursor, cursors, primary_old_data, old_data),
		}
	}
	
	fn delete_at(
		&mut self,
		primary_cursor: Cursor,
		cursors: &[Cursor]
	) {
		let mut bytes_deleted_so_far = 0;
		
		for cursor in cursors_in_order(primary_cursor, cursors) {
			let range = cursor.range();
			
			self.contents.drain(
				(range.start() - bytes_deleted_so_far)..=(range.end() - bytes_deleted_so_far)
			);
			
			// RangeInclusive<usize>::len() is unstable/nonexistant :/
			bytes_deleted_so_far += range.end() - range.start() + 1;
		}
		
		self.primary_cursor.head = min(
			min(primary_cursor.head, primary_cursor.tail),
			self.max_contents_index()
		);
		self.primary_cursor.collapse();
		
		self.cursors = cursors
			.iter()
			.map(|cursor| Cursor::at(min(cursor.lower_bound(), self.max_contents_index())))
			.collect();
		 
		self.combine_cursors_if_overlapping();
	}
	
	fn undo_delete_at(
		&mut self,
		primary_cursor: Cursor,
		cursors: &[Cursor],
		primary_old_data: &[u8],
		old_data: &[Vec<u8>]
	) {
		let primary_cursor_start = primary_cursor.lower_bound();
		
		self.contents.splice(
			primary_cursor_start..primary_cursor_start,
			primary_old_data.iter().copied()
		);
		
		for (cursor, old_data) in cursors.iter().zip(old_data) {
			let cursor_start = cursor.lower_bound();
			
			self.contents.splice(
				cursor_start..cursor_start,
				old_data.iter().copied()
			);
		}
		
		self.primary_cursor = primary_cursor;
		self.cursors = cursors.to_vec();
	}
	
	fn replace_at_with(
		&mut self,
		primary_cursor: Cursor,
		cursors: &[Cursor],
		new_byte: u8
	) {
		self.contents[primary_cursor.range()].fill(new_byte);
		
		for cursor in cursors {
			self.contents[cursor.range()].fill(new_byte);
		}
		
		self.primary_cursor = primary_cursor;
		self.cursors = cursors.to_vec();
	}
	
	fn undo_replace_at_with(
		&mut self,
		primary_cursor: Cursor,
		cursors: &[Cursor],
		primary_old_data: &[u8],
		old_data: &[Vec<u8>]
	) {
		self.contents.splice(
			primary_cursor.range(),
			primary_old_data.iter().copied()
		);
		
		for (cursor, old_data) in cursors.iter().zip(old_data) {
			self.contents.splice(
				cursor.range(),
				old_data.iter().copied()
			);
		}
		
		self.primary_cursor = primary_cursor;
		self.cursors = cursors.to_vec();
	}
}

fn cursors_in_order(
	primary_cursor: Cursor,
	cursors: &[Cursor]
) -> impl Iterator<Item=Cursor> {
	let primary_cursor_index = cursors
		.binary_search_by_key(&primary_cursor.head, |cursor| cursor.head)
		.unwrap_or_else(identity);
	
	cursors.iter()
		.copied()
		.take(primary_cursor_index)
		.chain(iter::once(primary_cursor))
		.chain(cursors.iter().copied().skip(primary_cursor_index))
}
