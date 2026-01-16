use crate::room::Room;

#[derive(Eq, Debug, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
pub enum Action {
    Move(Direction),
    Shoot(Direction),
    Climb,
    None
}

pub type Observation<'a> = &'a Room;

pub trait Agent {
    fn act(&mut self, obs: Observation) -> Action;
}
