use std::{
    collections::HashSet,
    fmt::{Display, Write},
    hash::Hash,
};

pub enum RoomKind {
    Void,
    Pit,
    Wumpus,
    Gold,
}

const MOVE_PENALTY: usize = 1;
const PIT_PENALTY: usize = 1000;
const WUMPUS_PENALTY: usize = 1000;
const GOLD_REWARD: usize = 1000;

impl RoomKind {
    pub fn score(&self) -> isize {
        match self {
            RoomKind::Void => -(MOVE_PENALTY as isize),
            RoomKind::Pit => -(PIT_PENALTY as isize),
            RoomKind::Wumpus => -(WUMPUS_PENALTY as isize),
            RoomKind::Gold => GOLD_REWARD as isize,
        }
    }
}

impl Display for RoomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomKind::Void => f.write_char('_'),
            RoomKind::Pit => f.write_char('P'),
            RoomKind::Wumpus => f.write_char('W'),
            RoomKind::Gold => f.write_char('G'),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum RoomSense {
    Stench,
    Breeze,
}

pub struct Room {
    kind: RoomKind,
    senses: HashSet<RoomSense>,
    visited: bool,
}

impl Room {
    pub fn new() -> Room {
        Room {
            kind: RoomKind::Void,
            senses: HashSet::new(),
            visited: false,
        }
    }

    pub fn set_kind(&mut self, kind: RoomKind) {
        self.kind = kind;
    }

    pub fn get_kind(&self) -> &RoomKind {
        &self.kind
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }

    pub fn add_sense(&mut self, sense: RoomSense) {
        self.senses.insert(sense);
    }

    /* pub fn remove_sense(&mut self, sense: RoomSense) {
        self.senses.remove(&sense);
    } */

    pub fn has_sense(&self, sense: RoomSense) -> bool {
        self.senses.contains(&sense)
    }
}
