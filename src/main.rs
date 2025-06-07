use std::{thread::sleep, time::Duration};

use arboard::Clipboard;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use rand::{Rng, rng};

enum TypePredicateResult {
    Continue,
    Skip,
    Break,
}

fn type_str<P>(enigo: &mut Enigo, text: &str, predicate: P)
where
    P: Fn(&mut Enigo, char) -> TypePredicateResult,
{
    for c in text.chars() {
        match predicate(enigo, c) {
            TypePredicateResult::Continue => {
                enigo.text(c.to_string().as_str()).unwrap();
                sleep(Duration::from_millis(25 + rng().random_range(0..20)));
            }
            TypePredicateResult::Skip => continue,
            TypePredicateResult::Break => break,
        }
    }
}

fn main() {
    let device_state = DeviceState::new();

    let default_predicate = |_: &mut Enigo, _: char| -> TypePredicateResult {
        if device_state.get_keys().contains(&Keycode::Escape) {
            TypePredicateResult::Break
        } else {
            TypePredicateResult::Continue
        }
    };

    loop {
        let keys = device_state.get_keys();

        if keys.contains(&Keycode::LControl) {
            if keys.contains(&Keycode::B) {
                // direct paste

                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                let mut clipboard = Clipboard::new().unwrap();
                let text = clipboard.get_text().unwrap();

                type_str(&mut enigo, &text, default_predicate);
                while device_state.get_keys().contains(&Keycode::B) {}
            } else if keys.contains(&Keycode::L) {
                // ignore line breaks in content and press enter twice

                let mut prefix_str: Option<&'static str> = None;
                if keys.contains(&Keycode::A) {
                    prefix_str = Some("/all ");
                }

                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                let mut clipboard = Clipboard::new().unwrap();
                let text = clipboard.get_text().unwrap();

                if let Some(prefix) = prefix_str {
                    type_str(&mut enigo, prefix, default_predicate);
                }

                type_str(&mut enigo, &text, |enigo, c| {
                    if device_state.get_keys().contains(&Keycode::Escape) {
                        TypePredicateResult::Break
                    } else {
                        match c {
                            '\r' => TypePredicateResult::Skip,
                            '\n' => {
                                enigo.key(Key::Return, Direction::Click).unwrap();
                                sleep(Duration::from_millis(40 + rng().random_range(0..5)));
                                enigo.key(Key::Return, Direction::Click).unwrap();
                                sleep(Duration::from_millis(40 + rng().random_range(0..5)));

                                if let Some(prefix) = prefix_str {
                                    type_str(enigo, prefix, default_predicate);
                                }

                                TypePredicateResult::Skip
                            }
                            _ => TypePredicateResult::Continue,
                        }
                    }
                });
                while device_state.get_keys().contains(&Keycode::L) {}
            }
        }

        sleep(Duration::from_millis(50));
    }
}
