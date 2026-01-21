use crate::room::Room;

#[derive(Eq, Debug, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Represents the possible actions an agent can take in the Wumpus World.
/// 
/// ## Variants
/// * `Move(Direction)` - Move the agent in the specified direction
/// * `Shoot(Direction)` - Shoot an arrow in the specified direction
/// * `Climb` - Climb out of the cave
/// * `None` - Perform no action
#[derive(PartialEq)]
pub enum Action {
    Move(Direction),
    Shoot(Direction),
    Climb,
    None
}

pub type Observation<'a> = &'a Room;

/// Trait for implementing an agent in the Wumpus World.
/// 
/// Types implementing this trait define the behavior of an agent
/// by determining what action to take based on the current observation.
/// 
/// ## Methods
/// * `act` - Determines the next action the agent should take given
/// the current observation
pub trait Agent {
    fn act(&mut self, obs: Observation) -> Action;
}
