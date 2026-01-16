use crate::agent::{Action, Agent, Direction, Observation};
use crate::tui;

pub struct KeyboardAgent {
    cmd: Option<String>,
}

impl KeyboardAgent {
    pub fn new() -> Self {
        KeyboardAgent { cmd: None }
    }

    pub fn set_cmd(&mut self, cmd: String) {
        self.cmd = Some(cmd);
    }
}

impl Agent for KeyboardAgent {
    fn act(&mut self, _obs: Observation) -> Action {
        match &self.cmd {
            Some(cmd) => {
                let mut direction: Option<Direction>;
                let action = match cmd.as_str() {
                    mov if mov.starts_with("mv ") && {
                        direction = tui::parse_direction(mov);
                        direction.is_some()
                    } =>
                    {
                        Action::Move(direction.unwrap())
                    }
                    shoot
                        if shoot.starts_with("sh ") && {
                            direction = tui::parse_direction(shoot);
                            direction.is_some()
                        } =>
                    {
                        Action::Shoot(direction.unwrap())
                    }
                    "cl" => Action::Climb,
                    _ => Action::None,
                };
                self.cmd = None;
                action
            }
            None => Action::None,
        }
    }
}
