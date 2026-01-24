use std::ops::{Add, Index, IndexMut};

use crate::{
    agent::Direction,
    env::Sense,
    room::{Room, RoomKind},
};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
}

impl Add<&Direction> for &Pos {
    type Output = Pos;

    fn add(self, rhs: &Direction) -> Self::Output {
        let mut result = self.clone();
        match rhs {
            Direction::North => result.row -= 1,
            Direction::South => result.row += 1,
            Direction::East => result.col += 1,
            Direction::West => result.col -= 1,
        };
        result
    }
}

pub struct Grid {
    cells: Vec<Vec<Room>>,
    nrows: usize,
    ncols: usize,
}

impl Grid {
    pub fn new(nrows: usize, ncols: usize) -> Grid {
        Grid {
            cells: (0..nrows)
                .map(|_| (0..ncols).map(|_| Room::new()).collect())
                .collect(),
            nrows,
            ncols,
        }
    }

    fn surround_cell(&mut self, row: usize, col: usize, with: &Sense) {
        if row as isize - 1 >= 0 {
            self.cells[row - 1][col].add_sense(with.clone());
        }
        if row + 1 <= self.nrows - 1 {
            self.cells[row + 1][col].add_sense(with.clone());
        }
        if col as isize - 1 >= 0 {
            self.cells[row][col - 1].add_sense(with.clone());
        }
        if col + 1 <= self.ncols - 1 {
            self.cells[row][col + 1].add_sense(with.clone());
        }
    }

    fn update_senses(&mut self) {
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                match self.cells[i][j].get_kind() {
                    RoomKind::Void => {}
                    RoomKind::Pit => {
                        // surround with breeze sense
                        self.surround_cell(i, j, &Sense::Breeze);
                    }
                    RoomKind::Wumpus => {
                        // surround with stench sense
                        self.surround_cell(i, j, &Sense::Stench);
                    }
                    RoomKind::Gold => {
                        self.cells[i][j].add_sense(Sense::Glitter);
                    }
                }
            }
        }
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn room_at(&self, position: &Pos) -> &Room {
        &self.cells[position.row][position.col]
    }

    pub fn mut_room_at(&mut self, position: &Pos) -> &mut Room {
        &mut self.cells[position.row][position.col]
    }

    pub fn initialize(&mut self) {
        self.update_senses();
        self.cells
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|room| room.set_visited(false)));
    }
}

////////////////////////////////////////////////////////////
/// Trait implementations for `Grid` ///////////////////////
////////////////////////////////////////////////////////////
impl Index<usize> for Grid {
    type Output = [Room];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}
////////////////////////////////////////////////////////////
