#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Unhandled,
}

impl Action {
    pub fn from_key(key: &str) -> Option<Action> {
        let up = Some(Action::Up);
        let down = Some(Action::Down);
        let left = Some(Action::Left);
        let right = Some(Action::Right);
        match key.trim() {
            "k" => up,
            "ArrowUp" => up,
            "z" => up,
            "ArrowDown" => down,
            "s" => down,
            "j" => down,
            "ArrowLeft" => left,
            "q" => left,
            "h" => left,
            "ArrowRight" => right,
            "d" => right,
            "l" => right,
            "Enter" => Some(Action::Confirm),
            _ => None,
        }
    }
}

