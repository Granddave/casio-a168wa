use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('a') => app.press_button_a(),
        KeyCode::Char('b') => app.press_button_b(),
        KeyCode::Char('c') => app.press_button_c(),
        KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
        KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
        _ => {}
    };
}
