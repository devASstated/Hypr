use anyhow::Result;
use evdev::{Device, InputEventKind, Key};
use std::path::Path;

use crate::state::KeyEvent;

pub struct InputHandler {
    device: Device,
}

impl InputHandler {
    pub fn new() -> Result<Self> {
        let device = find_keyboard_device()?;
        Ok(Self { device })
    }

    pub fn next_event(&mut self) -> Result<Option<KeyEvent>> {
        for ev in self.device.fetch_events()? {
            if let InputEventKind::Key(key) = ev.kind() {
                println!("Raw key: {:?}, value: {}", key, ev.value());
                let value = ev.value();

                // value: 1 = key down, 0 = key up, 2 = repeat
                match (key, value) {
                    (Key::KEY_LEFTALT | Key::KEY_RIGHTALT, 1) => {
                        return Ok(Some(KeyEvent::AltDown));
                    }

                    (Key::KEY_LEFTALT | Key::KEY_RIGHTALT, 0) => {
                        return Ok(Some(KeyEvent::AltUp));
                    }

                    (Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT, 1) => {
                        return Ok(Some(KeyEvent::ShiftDown));
                    }

                    (Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT, 0) => {
                        return Ok(Some(KeyEvent::ShiftUp));
                    }

                    (Key::KEY_TAB, 1) => {
                        return Ok(Some(KeyEvent::TabDown));
                    }

                    _ => {}
                }
            }
        }

        Ok(None)
    }
}

fn find_keyboard_device() -> Result<Device> {
    let device = Device::open("/dev/input/event4")?;
    println!(
        "Using keyboard device: {}",
        device.name().unwrap_or("Unknown")
    );
    Ok(device)
}


//
// fn find_keyboard_device() -> Result<Device> {
//     for entry in fs::read_dir("/dev/input")? {
//         let path = entry?.path();
//
//         if !is_event_device(&path) {
//             continue;
//         }
//
//         if let Ok(device) = Device::open(&path) {
//             if let Some(keys) = device.supported_keys() {
//                 if keys.contains(Key::KEY_TAB)
//                     && keys.contains(Key::KEY_LEFTALT)
//                 {
//                     println!(
//                         "Using keyboard device: {}",
//                         device.name().unwrap_or("Unknown")
//                     );
//                     return Ok(device);
//                 }
//             }
//         }
//     }
//
//     Err(anyhow::anyhow!("No suitable keyboard device found"))
// }


fn is_event_device(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        name.to_string_lossy().starts_with("event")
    } else {
        false
    }
}

