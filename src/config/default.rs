use crate::{action::{AppAction, BufferAction, CursorAction}, buffer::{Mode, PartialAction}, config::Config};

impl Default for Config {
	#[allow(clippy::too_many_lines)]
	fn default() -> Self {
		use AppAction::*;
		use BufferAction::*;
		use CursorAction::*;
		
		[
			(Mode::Normal, [
				(None, [
					("q".try_into().unwrap(), QuitIfSaved.into()),
					("Q".try_into().unwrap(), Quit.into()),
					
					("v".try_into().unwrap(), SelectMode.into()),
					
					("g".try_into().unwrap(), Goto.into()),
					("z".try_into().unwrap(), View.into()),
					("r".try_into().unwrap(), Replace.into()),
					(" ".try_into().unwrap(), Space.into()),
					("*".try_into().unwrap(), Repeat.into()),
					("t".try_into().unwrap(), To.into()),
					
					("i".try_into().unwrap(), MoveByteUp.into()),
					("k".try_into().unwrap(), MoveByteDown.into()),
					("j".try_into().unwrap(), MoveByteLeft.into()),
					("l".try_into().unwrap(), MoveByteRight.into()),
					
					("up".try_into().unwrap(), MoveByteUp.into()),
					("down".try_into().unwrap(), MoveByteDown.into()),
					("left".try_into().unwrap(), MoveByteLeft.into()),
					("right".try_into().unwrap(), MoveByteRight.into()),
					
					("G".try_into().unwrap(), GotoFileEnd.into()),
					
					("C-e".try_into().unwrap(), ScrollDown.into()),
					("C-y".try_into().unwrap(), ScrollUp.into()),
					
					("C-d".try_into().unwrap(), PageCursorHalfDown.into()),
					("C-u".try_into().unwrap(), PageCursorHalfUp.into()),
					
					("C-f".try_into().unwrap(), PageDown.into()),
					("C-b".try_into().unwrap(), PageUp.into()),
					
					("w".try_into().unwrap(), MoveNextWordStart.into()),
					("e".try_into().unwrap(), MoveNextWordEnd.into()),
					("b".try_into().unwrap(), MovePreviousWordStart.into()),
					
					(";".try_into().unwrap(), CollapseSelection.into()),
					("A-;".try_into().unwrap(), FlipSelections.into()),
					
					("x".try_into().unwrap(), ExtendLineBelow.into()),
					("X".try_into().unwrap(), ExtendLineAbove.into()),
					
					("d".try_into().unwrap(), Delete.into()),
					
					("u".try_into().unwrap(), Undo.into()),
					("U".try_into().unwrap(), Redo.into()),
					
					("C-j".try_into().unwrap(), PreviousBuffer.into()),
					("C-l".try_into().unwrap(), NextBuffer.into()),
					
					("C".try_into().unwrap(), CopySelectionOnNextLine.into()),
					
					("(".try_into().unwrap(), RotateSelectionsBackward.into()),
					(")".try_into().unwrap(), RotateSelectionsForward.into()),
					
					(",".try_into().unwrap(), KeepPrimarySelection.into()),
					("A-,".try_into().unwrap(), RemovePrimarySelection.into()),
					
					("1".try_into().unwrap(), SplitSelectionsInto1s.into()),
					("2".try_into().unwrap(), SplitSelectionsInto2s.into()),
					("3".try_into().unwrap(), SplitSelectionsInto3s.into()),
					("4".try_into().unwrap(), SplitSelectionsInto4s.into()),
					("5".try_into().unwrap(), SplitSelectionsInto5s.into()),
					("6".try_into().unwrap(), SplitSelectionsInto6s.into()),
					("7".try_into().unwrap(), SplitSelectionsInto7s.into()),
					("8".try_into().unwrap(), SplitSelectionsInto8s.into()),
					("9".try_into().unwrap(), SplitSelectionsInto9s.into()),
					
					("J".try_into().unwrap(), JumpToSelectedOffsetRelativeToMark.into()),
					("A-J".try_into().unwrap(), JumpToSelectedOffset.into()),
					
					("m".try_into().unwrap(), ToggleMark.into()),
					
					("y".try_into().unwrap(), Yank.into()),
					
					("C- ".try_into().unwrap(), InspectSelection.into()),
					("A- ".try_into().unwrap(), InspectSelectionColor.into()),
				].into()),
				(Some(PartialAction::Goto), [
					("j".try_into().unwrap(), GotoLineStart.into()),
					("l".try_into().unwrap(), GotoLineEnd.into()),
					
					("g".try_into().unwrap(), GotoFileStart.into()),
				].into()),
				(Some(PartialAction::View), [
					("z".try_into().unwrap(), AlignViewCenter.into()),
					("b".try_into().unwrap(), AlignViewBottom.into()),
					("t".try_into().unwrap(), AlignViewTop.into()),
				].into()),
				(Some(PartialAction::Space), [
					("w".try_into().unwrap(), Save.into()),
				].into()),
				(Some(PartialAction::Repeat), [
					("i".try_into().unwrap(), MoveByteUp.into()),
					("k".try_into().unwrap(), MoveByteDown.into()),
					("j".try_into().unwrap(), MoveByteLeft.into()),
					("l".try_into().unwrap(), MoveByteRight.into()),
					
					("up".try_into().unwrap(), MoveByteUp.into()),
					("down".try_into().unwrap(), MoveByteDown.into()),
					("left".try_into().unwrap(), MoveByteLeft.into()),
					("right".try_into().unwrap(), MoveByteRight.into()),
					
					("C-e".try_into().unwrap(), ScrollDown.into()),
					("C-y".try_into().unwrap(), ScrollUp.into()),
					
					("C-d".try_into().unwrap(), PageCursorHalfDown.into()),
					("C-u".try_into().unwrap(), PageCursorHalfUp.into()),
					
					("C-f".try_into().unwrap(), PageDown.into()),
					("C-b".try_into().unwrap(), PageUp.into()),
					
					("w".try_into().unwrap(), MoveNextWordStart.into()),
					("e".try_into().unwrap(), MoveNextWordEnd.into()),
					("b".try_into().unwrap(), MovePreviousWordStart.into()),
					
					("x".try_into().unwrap(), ExtendLineBelow.into()),
					("X".try_into().unwrap(), ExtendLineAbove.into()),
					
					("d".try_into().unwrap(), Delete.into()),
					
					("C".try_into().unwrap(), CopySelectionOnNextLine.into()),
				].into()),
				(Some(PartialAction::To), [
					("m".try_into().unwrap(), ExtendToMark.into()),
					("0".try_into().unwrap(), ExtendToNull.into()),
					("f".try_into().unwrap(), ExtendToFF.into()),
				].into()),
			].into()),
			(Mode::Select, [
				(None, [
					("q".try_into().unwrap(), QuitIfSaved.into()),
					("Q".try_into().unwrap(), Quit.into()),
					
					("v".try_into().unwrap(), NormalMode.into()),
					
					("g".try_into().unwrap(), Goto.into()),
					("z".try_into().unwrap(), View.into()),
					("r".try_into().unwrap(), Replace.into()),
					(" ".try_into().unwrap(), Space.into()),
					("*".try_into().unwrap(), Repeat.into()),
					("t".try_into().unwrap(), To.into()),
					
					("i".try_into().unwrap(), ExtendByteUp.into()),
					("k".try_into().unwrap(), ExtendByteDown.into()),
					("j".try_into().unwrap(), ExtendByteLeft.into()),
					("l".try_into().unwrap(), ExtendByteRight.into()),
					
					("up".try_into().unwrap(), ExtendByteUp.into()),
					("down".try_into().unwrap(), ExtendByteDown.into()),
					("left".try_into().unwrap(), ExtendByteLeft.into()),
					("right".try_into().unwrap(), ExtendByteRight.into()),
					
					("C-e".try_into().unwrap(), ScrollDown.into()),
					("C-y".try_into().unwrap(), ScrollUp.into()),
					
					("C-d".try_into().unwrap(), PageCursorHalfDown.into()),
					("C-u".try_into().unwrap(), PageCursorHalfUp.into()),
					
					("C-f".try_into().unwrap(), PageDown.into()),
					("C-b".try_into().unwrap(), PageUp.into()),
					
					("w".try_into().unwrap(), ExtendNextWordStart.into()),
					("e".try_into().unwrap(), ExtendNextWordEnd.into()),
					("b".try_into().unwrap(), ExtendPreviousWordStart.into()),
					
					(";".try_into().unwrap(), CollapseSelection.into()),
					("A-;".try_into().unwrap(), FlipSelections.into()),
					
					("x".try_into().unwrap(), ExtendLineBelow.into()),
					("X".try_into().unwrap(), ExtendLineAbove.into()),
					
					("d".try_into().unwrap(), Delete.into()),
					
					("u".try_into().unwrap(), Undo.into()),
					("U".try_into().unwrap(), Redo.into()),
					
					("C".try_into().unwrap(), CopySelectionOnNextLine.into()),
					
					("(".try_into().unwrap(), RotateSelectionsBackward.into()),
					(")".try_into().unwrap(), RotateSelectionsForward.into()),
					
					(",".try_into().unwrap(), KeepPrimarySelection.into()),
					("A-,".try_into().unwrap(), RemovePrimarySelection.into()),
					
					("1".try_into().unwrap(), SplitSelectionsInto1s.into()),
					("2".try_into().unwrap(), SplitSelectionsInto2s.into()),
					("3".try_into().unwrap(), SplitSelectionsInto3s.into()),
					("4".try_into().unwrap(), SplitSelectionsInto4s.into()),
					("5".try_into().unwrap(), SplitSelectionsInto5s.into()),
					("6".try_into().unwrap(), SplitSelectionsInto6s.into()),
					("7".try_into().unwrap(), SplitSelectionsInto7s.into()),
					("8".try_into().unwrap(), SplitSelectionsInto8s.into()),
					("9".try_into().unwrap(), SplitSelectionsInto9s.into()),
					
					("J".try_into().unwrap(), JumpToSelectedOffsetRelativeToMark.into()),
					("A-J".try_into().unwrap(), JumpToSelectedOffset.into()),
					
					("m".try_into().unwrap(), ToggleMark.into()),
					
					("y".try_into().unwrap(), Yank.into()),
					
					("C- ".try_into().unwrap(), InspectSelection.into()),
					("A- ".try_into().unwrap(), InspectSelectionColor.into()),
				].into()),
				(Some(PartialAction::View), [
					("z".try_into().unwrap(), AlignViewCenter.into()),
					("b".try_into().unwrap(), AlignViewBottom.into()),
					("t".try_into().unwrap(), AlignViewTop.into()),
				].into()),
				(Some(PartialAction::Space), [
					("w".try_into().unwrap(), Save.into()),
				].into()),
				(Some(PartialAction::Repeat), [
					("i".try_into().unwrap(), ExtendByteUp.into()),
					("k".try_into().unwrap(), ExtendByteDown.into()),
					("j".try_into().unwrap(), ExtendByteLeft.into()),
					("l".try_into().unwrap(), ExtendByteRight.into()),
					
					("up".try_into().unwrap(), ExtendByteUp.into()),
					("down".try_into().unwrap(), ExtendByteDown.into()),
					("left".try_into().unwrap(), ExtendByteLeft.into()),
					("right".try_into().unwrap(), ExtendByteRight.into()),
					
					("C-e".try_into().unwrap(), ScrollDown.into()),
					("C-y".try_into().unwrap(), ScrollUp.into()),
					
					("C-d".try_into().unwrap(), PageCursorHalfDown.into()),
					("C-u".try_into().unwrap(), PageCursorHalfUp.into()),
					
					("C-f".try_into().unwrap(), PageDown.into()),
					("C-b".try_into().unwrap(), PageUp.into()),
					
					("w".try_into().unwrap(), ExtendNextWordStart.into()),
					("e".try_into().unwrap(), ExtendNextWordEnd.into()),
					("b".try_into().unwrap(), ExtendPreviousWordStart.into()),
					
					("x".try_into().unwrap(), ExtendLineBelow.into()),
					("X".try_into().unwrap(), ExtendLineAbove.into()),
					
					("d".try_into().unwrap(), Delete.into()),
					
					("C".try_into().unwrap(), CopySelectionOnNextLine.into()),
				].into()),
				(Some(PartialAction::To), [
					("m".try_into().unwrap(), ExtendToMark.into()),
					("0".try_into().unwrap(), ExtendToNull.into()),
					("f".try_into().unwrap(), ExtendToFF.into()),
				].into()),
			].into())
		].into()
	}
}
