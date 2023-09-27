use std::collections::VecDeque;

use enigo::{Enigo, Key, KeyboardControllable};

use crate::err;

#[derive(Debug, Clone)]
pub enum KeyPress {
    Single(enigo::Key),
    String(String),
}
fn key_from_str(key: &str) -> Option<enigo::Key> {
    match key {
        #[cfg(target_os = "macos")]
        "missioncontrol" => Some(Key::MissionControl),
        "ctrl" | "control" => Some(Key::Control),
        "alt" | "option" => {
            #[cfg(target_os = "windows")]
            return Some(Key::Alt);
            #[cfg(target_os = "linux")]
            return Some(Key::Alt);
            #[cfg(target_os = "macos")]
            return Some(Key::Option);
        }
        "super" | "windows" | "meta" | "command" => Some(Key::Control),
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),
        _ => None,
    }
}

pub fn parse(input: &str) -> Result<VecDeque<KeyPress>, Box<dyn std::error::Error + Send + Sync>> {
    let mut col = 0;
    let arr: VecDeque<&str> = input.split(" + ").map(|s| s.trim()).collect();
    let mut keycombo: VecDeque<KeyPress> = VecDeque::new();
    for k in arr {
        if let Some(code) = k.strip_prefix(':') {
            let s = match key_from_str(code) {
                Some(c) => c,
                None => {
                    return Err(err::err(err::SuperlightError::Parsing(format!(
                        "Could not parse key at column {col}\n\t\"{input}\"\n\t {}^",
                        " ".repeat(col)
                    ))))
                }
            };
            keycombo.push_back(KeyPress::Single(s));
        } else {
            keycombo.push_back(KeyPress::String(k.to_owned()));
        };
        col += 3 + k.chars().count();
    }
    Ok(keycombo)
}

pub fn execute(enigo: &mut Enigo, mut keys: VecDeque<KeyPress>) {
    if keys.len() > 1 {
        let p = match keys.pop_front() {
            Some(e) => e,
            None => todo!(),
        };
        match p {
            KeyPress::Single(s) => {
                enigo.key_down(s);
                execute(enigo, keys);
                enigo.key_up(s);
            }
            KeyPress::String(s) => {
                enigo.key_sequence(&s);
                execute(enigo, keys)
            }
        }
    } else {
        let p = match keys.pop_front() {
            Some(e) => e,
            None => todo!(),
        };
        match p {
            KeyPress::Single(s) => {
                enigo.key_click(s)
            }
            KeyPress::String(s) => {
                enigo.key_sequence(&s);
                execute(enigo, keys)
            }
        }
    }
}
