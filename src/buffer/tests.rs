#![allow(clippy::unwrap_used, reason = "tests")]

use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::Buffer;

#[test]
fn only_one_history_entry() {
    let mut x = Buffer::from("abcabc");
    x.update(&Event::Key(KeyEvent::from(KeyCode::Char('f'))));
    assert_eq!(x.history.as_vec(), &[Box::from("abcabc")]);
    x.update(&Event::Key(KeyEvent::from(KeyCode::Char('c'))));
    assert_eq!(x.history.as_vec(), &[Box::from("abcabc")]);
}
