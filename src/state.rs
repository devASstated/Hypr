#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Idle,
    AltHeld,
    WorkspaceMode,
}

#[derive(Debug)]
pub enum KeyEvent {
    AltDown,
    AltUp,
    TabDown,
    ShiftDown,
    ShiftUp,
}

#[derive(Debug)]
pub enum Action {
    WorkspacePrevMRU,
    WorkspaceNextCyclic,
    WorkspacePrevCyclic,
}

pub struct State {
    mode: Mode,
    alt_held: bool,
    shift_held: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Idle,
            alt_held: false,
            shift_held: false,
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) -> Option<Action> {
        match event {
            KeyEvent::AltDown => {
                self.alt_held = true;
                self.mode = Mode::AltHeld;
                None
            }

            KeyEvent::AltUp => {
                self.alt_held = false;
                self.mode = Mode::Idle;
                None
            }

            KeyEvent::ShiftDown => {
                self.shift_held = true;
                None
            }

            KeyEvent::ShiftUp => {
                self.shift_held = false;
                None
            }

            KeyEvent::TabDown => {
                if !self.alt_held {
                    return None;
                }

                match self.mode {
                    Mode::AltHeld => {
                        self.mode = Mode::WorkspaceMode;
                        Some(Action::WorkspacePrevMRU)
                    }

                    Mode::WorkspaceMode => {
                        if self.shift_held {
                            Some(Action::WorkspacePrevCyclic)
                        } else {
                            Some(Action::WorkspaceNextCyclic)
                        }
                    }

                    Mode::Idle => None,
                }
            }
        }
    }
}

