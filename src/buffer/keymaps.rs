use crate::Mode;

/// Action to be done on the buffer
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Delete
    Delete(GoToAction, Option<GoToAction>),
    /// Deletes the whole line
    DeleteLine,
    /// Deletes the char after the cursor
    DeleteNextChar,
    /// Deletes the char before the cursor
    DeletePreviousChar,
    /// Action to move the cursor to a location denotated by a condition
    GoTo(GoToAction),
    /// Inserts a char in the buffer
    InsertChar(char),
    /// Undo the last undo action
    Redo,
    /// Replace the char under the cursor with
    ReplaceWith(char),
    /// Switches to a new mode
    SelectMode(Mode),
    /// Capitalises minuscules and lowers capitals
    ToggleCapitalisation,
    /// Undo the last edition
    Undo,
}

impl From<GoToAction> for Action {
    fn from(value: GoToAction) -> Self {
        Self::GoTo(value)
    }
}

impl From<Mode> for Action {
    fn from(value: Mode) -> Self {
        Self::SelectMode(value)
    }
}

/// Actions to move the cursor
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GoToAction {
    /// Beginning of line (column 0), reached with `0`
    BeginningOfLine,
    /// Move to the beginning of the previous WORD
    BeginningOfWORD,
    /// Move to the beginning of the previous word
    BeginningOfWord,
    /// End of line, like with `$` and `A`
    EndOfLine,
    /// Move to the end of the previous WORD, reached with `gE`
    EndOfPreviousWORD,
    /// Move to the end of the previous word, reached with `ge`
    EndOfPreviousWord,
    /// End of current or next WORD, reached with `E`
    EndWORD,
    /// End of current or next word, reached with `e`
    EndWord,
    /// First non space character, like with `I` and `^`
    FirstNonSpace,
    /// Move the cursor left by one character
    Left,
    /// Find next occurrence of char and place cursor on it
    NextOccurrenceOf(char),
    /// Move to the beginning of the next WORD
    NextWORD,
    /// Move to the beginning of the next word
    NextWord,
    /// Find previous occurrence of char and place cursor on it
    PreviousOccurrenceOf(char),
    /// Move the cursor right by one character
    Right,
}

/// Action that is pending for another keypress
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPending {
    /// Pending action that only requires 1 character to form a goto action.
    ///
    /// Combinable with delete, see [`Self::DeleteAction`].
    CombinablePending(CombinablePending),
    /// Delete pending (`d` pressed, waiting for go-to action)
    Delete,
    /// Delete pending and go-to action pending too (e.g. `df` pressed).
    DeleteAction(CombinablePending),
    /// Applies a single char action to a motion.
    GoTo,
    /// Replace one character
    ReplaceOne,
}

/// Pending action that only requires 1 character to form a goto action.
///
/// Combinable with delete, see [`OPending::DeleteAction`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CombinablePending {
    /// Find next char that is equal to...
    FindNext,
    /// Find next char that is equal to... and decrement
    FindNextDecrement,
    /// Find previous char that is equal to...
    FindPrevious,
    /// Find previous char that is equal to... and increment
    FindPreviousIncrement,
}
