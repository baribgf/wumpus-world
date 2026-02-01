use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use crate::env::Sense;

#[derive(PartialEq)]
pub enum RoomKind {
    Void,
    Pit,
    Wumpus,
    Gold,
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

pub struct Room {
    kind: RoomKind,
    senses: HashSet<Sense>,
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

    pub fn senses(&self) -> &HashSet<Sense> {
        &self.senses
    }

    pub fn mut_senses(&mut self) -> &mut HashSet<Sense> {
        &mut self.senses
    }

    pub fn add_sense(&mut self, sense: Sense) {
        if sense != Sense::Breeze && sense != Sense::Stench && sense != Sense::Glitter {
            panic!("{:?} is not a room sense!", sense);
        }

        self.senses.insert(sense);
    }

    pub fn has_sense(&self, sense: Sense) -> bool {
        self.senses.contains(&sense)
    }
}
