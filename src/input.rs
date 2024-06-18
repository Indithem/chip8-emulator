pub struct InpuState {
    /// The state of all the keys that are pressed/releasd
    /// at the current moment.
    registry_stack: Vec<Key>,

    rx: Receiver<(Key, ElementState)>,
}

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Key{
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
    A, B, C, D, E, F
}

impl InpuState {
    pub fn new(rx: Receiver<(Key, ElementState)>) -> Self {
        InpuState {
            registry_stack: Vec::with_capacity(16),
            rx,
        }
    }

    /// Updates, according to the keypresses
    fn update(&mut self, (key, state): (Key, ElementState)) {
        if state.is_pressed() {
            self.registry_stack.push(key);
        } else {
            self.registry_stack.retain(|&k| k != key);
        }

        // just a check if we are not holding more than one copy of a key
        if self.registry_stack.iter().filter(|&k| *k == key).count() > 1 {
            tracing::warn!("Holding more than one copy of a keypress: {:?}", key);
        }
    }

    /// Fills the buffer with the keypresses that happend recently
    pub fn receive_keys(&mut self) {
        while let Ok(input) = self.rx.try_recv() {
            self.update(input)
        }
    }

    /// waits till a new press
    /// and returns the key that was next fifo pressed
    pub fn wait_for_key(&mut self) -> Key {
        let current_pressed = self.registry_stack.first().cloned();
        loop {
            if let Ok(input) = self.rx.recv() {
                self.update(input);
                let new_press = self.registry_stack.first().cloned();
                if let Some(_) = new_press {
                    if new_press != current_pressed {
                        return new_press.unwrap();
                    }
                }
            }
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.registry_stack.contains(&key)
    }
}

impl Key {
    pub fn from_key_code(key:KeyCode) -> Option<Key> {
        Some(match key {
            KeyCode::Numpad0 => Key::Zero,
            KeyCode::Numpad1 => Key::One,
            KeyCode::Numpad2 => Key::Two,
            KeyCode::Numpad3 => Key::Three,
            KeyCode::Numpad4 => Key::Four,
            KeyCode::Numpad5 => Key::Five,
            KeyCode::Numpad6 => Key::Six,
            KeyCode::Numpad7 => Key::Seven,
            KeyCode::Numpad8 => Key::Eight,
            KeyCode::Numpad9 => Key::Nine,
            KeyCode::KeyA => Key::A,
            KeyCode::KeyB => Key::B,
            KeyCode::KeyC => Key::C,
            KeyCode::KeyD => Key::D,
            KeyCode::KeyE => Key::E,
            KeyCode::KeyF => Key::F,
            _ => {
                tracing::error!("Unknown key: {:?}", key);
                return None;
            },
        })
    }
}

impl TryFrom<u8> for Key {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x0 => Key::Zero,
            0x1 => Key::One,
            0x2 => Key::Two,
            0x3 => Key::Three,
            0x4 => Key::Four,
            0x5 => Key::Five,
            0x6 => Key::Six,
            0x7 => Key::Seven,
            0x8 => Key::Eight,
            0x9 => Key::Nine,
            0xA => Key::A,
            0xB => Key::B,
            0xC => Key::C,
            0xD => Key::D,
            0xE => Key::E,
            0xF => Key::F,
            _ => {
                tracing::error!("Unknown key: {:?}", value);
                return Err(());
            },
        })
    }
}

impl Into<u8> for Key{
    fn into(self) -> u8 {
        self as u8
    }
}

use std::sync::mpsc::Receiver;

use winit::{event::ElementState, keyboard::KeyCode};
