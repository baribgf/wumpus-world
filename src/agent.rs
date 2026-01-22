use std::collections::HashSet;

use crate::{grid::Pos, room::RoomSense};

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
}

#[derive(Debug)]
pub enum Sense {
    Room(HashSet<RoomSense>),
    Scream,
    Bump,
    Ceil,
    None,
}

#[derive(Debug)]
pub struct Observation {
    position: Pos,
    sense: Sense
}

impl Observation {
    pub fn new(position: Pos, sense: Sense) -> Self {
        Self { position, sense }
    }

    pub fn position(&self) -> &Pos {
        &self.position
    }

    pub fn set_position(&mut self, pos: Pos) {
        self.position = pos
    }

    pub fn sense(&self) -> &Sense {
        &self.sense
    }

    pub fn set_sense(&mut self, sense: Sense) {
        self.sense = sense
    }
}

/// Trait for implementing an agent in the Wumpus World.
///
/// Types implementing this trait define the behavior of an agent
/// by determining what action to take based on the current observation.
///
/// ## Methods
/// * `act` - Determines the next action the agent should take given
/// the current observation
pub trait Agent {
    fn act(&mut self, obs: &Observation) -> Action;
}
