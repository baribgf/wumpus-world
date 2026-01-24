use std::collections::HashSet;

use crate::{env::Sense, grid::Pos};

#[derive(Eq, Debug, PartialEq, Hash, Clone)]
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
#[derive(PartialEq)]
pub enum Action {
    Move(Direction),
    Shoot(Direction),
    Climb,
}

#[derive(Debug)]
pub struct Observation {
    position: Pos,
    directions: HashSet<Direction>,
    senses: HashSet<Sense>,
}

impl Observation {
    pub fn new(position: Pos) -> Self {
        Self {
            position,
            directions: HashSet::new(),
            senses: HashSet::new(),
        }
    }

    pub fn position(&self) -> &Pos {
        &self.position
    }

    pub fn set_position(&mut self, pos: Pos) {
        self.position = pos
    }

    pub fn directions(&self) -> &HashSet<Direction> {
        &self.directions
    }

    pub fn mut_directions(&mut self) -> &mut HashSet<Direction> {
        &mut self.directions
    }

    pub fn senses(&self) -> &HashSet<Sense> {
        &self.senses
    }

    pub fn mut_senses(&mut self) -> &mut HashSet<Sense> {
        &mut self.senses
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
