use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use rand::seq::IteratorRandom;

use crate::{
    agent::{Action, Direction, Observation},
    grid::{Grid, Pos},
    room::{Room, RoomKind},
};

const ARROW_PENALTY: usize = 10;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Sense {
    Stench,
    Breeze,
    Scream(Direction),
    Glitter,
    Bump(Direction),
    Ceil,
}

#[derive(Debug)]
pub enum ActionResult {
    Ok,
    GameOver,
    Sense(Sense),
}

pub struct Environment {
    grid: Grid,
    score: isize,
    init_pos: Pos,
    agent_pos: Pos,
    curr_obs: Observation,
}

pub enum GridType {
    Classic,
    Random,
}

impl Environment {
    pub fn new(grid_type: GridType) -> Self {
        let mut grid: Grid;
        let init_pos: Pos;

        match grid_type {
            GridType::Classic => {
                init_pos = Pos::new(3, 0);

                grid = Grid::new(4, 4);

                grid[0][0].set_kind(RoomKind::Void);
                grid[0][1].set_kind(RoomKind::Void);
                grid[0][2].set_kind(RoomKind::Void);
                grid[0][3].set_kind(RoomKind::Pit);

                grid[1][0].set_kind(RoomKind::Wumpus);
                grid[1][1].set_kind(RoomKind::Gold);
                grid[1][2].set_kind(RoomKind::Pit);
                grid[1][3].set_kind(RoomKind::Void);

                grid[2][0].set_kind(RoomKind::Void);
                grid[2][1].set_kind(RoomKind::Void);
                grid[2][2].set_kind(RoomKind::Void);
                grid[2][3].set_kind(RoomKind::Void);

                grid[3][0].set_kind(RoomKind::Void);
                grid[3][1].set_kind(RoomKind::Void);
                grid[3][2].set_kind(RoomKind::Pit);
                grid[3][3].set_kind(RoomKind::Void);
            }
            GridType::Random => {
                const MAX_NROWS: usize = 6;
                const MAX_NCOLS: usize = 6;
                const PIT_PROB: f64 = 0.2;

                let nrows: usize = rand::random_range(4..=MAX_NROWS);
                let ncols: usize = rand::random_range(4..=MAX_NCOLS);

                grid = Grid::new(nrows, ncols);

                /* Initialize all rooms with type `RoomKind::Void` */
                // cp_set: Candidate Positions Set
                let mut cp_set: HashSet<Pos> = HashSet::new();
                for i in 0..nrows {
                    for j in 0..ncols {
                        cp_set.insert(Pos::new(i, j));
                        grid[i][j].set_kind(RoomKind::Void);
                    }
                }

                let mut rng = rand::rng();
                /* Choose a random start position */
                init_pos = cp_set.iter().choose(&mut rng).unwrap().clone();
                cp_set.remove(&init_pos);

                cp_set.remove(&Pos::new(
                    (init_pos.row as isize - 1).max(0) as usize,
                    init_pos.col,
                ));
                cp_set.remove(&Pos::new(init_pos.row + 1, init_pos.col));
                cp_set.remove(&Pos::new(
                    init_pos.row,
                    (init_pos.col as isize - 1).max(0) as usize,
                ));
                cp_set.remove(&Pos::new(init_pos.row, init_pos.col + 1));

                /* Choose random positions that are not the start position
                to put `RoomKind::Pit` rooms with probability `PIT_PROB` */
                let mut pits: HashSet<Pos> = HashSet::new();
                for pos in cp_set.iter() {
                    if rand::random_bool(PIT_PROB) {
                        grid.mut_room_at(&pos).set_kind(RoomKind::Pit);
                        pits.insert(pos.clone());
                    }
                }
                cp_set.retain(|pos| !pits.contains(pos));

                /* Choose a random position that is not the start
                position and doesn't containt a pit to put the Wumpus */
                let wumpus_pos = cp_set.iter().choose(&mut rng).unwrap();
                grid.mut_room_at(&wumpus_pos).set_kind(RoomKind::Wumpus);

                /* Choose a random position that is not the start
                position and contains neither a pit or wumpus
                to put the Gold! */
                let gold_pos = cp_set.iter().choose(&mut rng).unwrap();
                grid.mut_room_at(&gold_pos).set_kind(RoomKind::Gold);
            }
        }

        let mut env = Environment {
            grid,
            score: 0,
            init_pos: init_pos.clone(),
            agent_pos: init_pos.clone(),
            curr_obs: Observation::new(init_pos.clone()),
        };

        Self::initialize(&mut env);

        env
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn agent_position(&self) -> &Pos {
        &self.agent_pos
    }

    pub fn mut_agent_position(&mut self) -> &mut Pos {
        &mut self.agent_pos
    }

    pub fn is_direction_valid(&self, direction: &Direction) -> bool {
        match direction {
            Direction::North => self.agent_pos.row > 0,
            Direction::South => self.agent_pos.row < self.grid.nrows() - 1,
            Direction::East => self.agent_pos.col < self.grid.ncols() - 1,
            Direction::West => self.agent_pos.col > 0,
        }
    }

    fn available_directions(&self) -> HashSet<Direction> {
        let mut all_directions = HashSet::new();
        all_directions.insert(Direction::North);
        all_directions.insert(Direction::South);
        all_directions.insert(Direction::East);
        all_directions.insert(Direction::West);

        all_directions.retain(|d| self.is_direction_valid(d));

        all_directions
    }

    fn current_room_senses(&self) -> HashSet<Sense> {
        self.current_room()
            .senses()
            .iter()
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn score(&self) -> isize {
        self.score
    }

    pub fn set_score(&mut self, score: isize) {
        self.score = score;
    }

    pub fn current_room(&self) -> &Room {
        &self.grid.room_at(&self.agent_pos)
    }

    pub fn observation(&self) -> &Observation {
        &self.curr_obs
    }

    pub fn lightup_agent_position(&mut self) {
        self.grid
            .mut_room_at(&self.agent_position().clone())
            .set_visited(true);
    }

    pub fn initialize(&mut self) {
        self.score = 0;
        self.agent_pos = self.init_pos.clone();
        self.curr_obs.set_position(self.init_pos.clone());
        self.curr_obs.mut_senses().clear();
        self.curr_obs.mut_directions().clear();
        let directions = self.available_directions();
        self.curr_obs.mut_directions().extend(directions);
        self.grid.initialize();
        self.lightup_agent_position();
    }

    pub fn step(&mut self, action: &Action) -> ActionResult {
        match action {
            Action::Move(direction) => {
                if !self.is_direction_valid(&direction) {
                    return ActionResult::Sense(Sense::Bump(direction.clone()));
                }
                match direction {
                    Direction::North => {
                        self.mut_agent_position().row -= 1;
                    }
                    Direction::South => {
                        self.mut_agent_position().row += 1;
                    }
                    Direction::East => {
                        self.mut_agent_position().col += 1;
                    }
                    Direction::West => {
                        self.mut_agent_position().col -= 1;
                    }
                }
                self.lightup_agent_position();
                self.set_score(self.score() + self.current_room().get_kind().score());

                // update current observation
                self.curr_obs.set_position(self.agent_position().clone());

                let current_senses = self.current_room_senses();
                self.curr_obs.mut_senses().clear();
                self.curr_obs.mut_senses().extend(current_senses);

                let available_dirs = self.available_directions();
                self.curr_obs.mut_directions().clear();
                self.curr_obs.mut_directions().extend(available_dirs);

                match self.current_room().get_kind() {
                    RoomKind::Pit => ActionResult::GameOver,
                    RoomKind::Wumpus => ActionResult::GameOver,
                    _ => ActionResult::Ok,
                }
            }
            Action::Shoot(direction) => {
                if !self.is_direction_valid(&direction) {
                    return ActionResult::Sense(Sense::Bump(direction.clone()));
                }

                let target_position = self.agent_position() + &direction;
                match self.grid.room_at(&target_position).get_kind() {
                    RoomKind::Wumpus => {
                        self.grid
                            .mut_room_at(&target_position)
                            .set_kind(RoomKind::Void);
                        self.grid.mut_room_at(&target_position).set_visited(true);
                        self.set_score(self.score() - ARROW_PENALTY as isize);

                        // update current observation
                        let sense = Sense::Scream(direction.clone());
                        self.curr_obs.mut_senses().insert(sense.clone());

                        ActionResult::Sense(sense)
                    }
                    _ => ActionResult::Ok,
                }
            }
            Action::Climb => match self.agent_pos == self.init_pos {
                true => ActionResult::GameOver,
                false => {
                    let sense = Sense::Ceil;
                    self.curr_obs.mut_senses().insert(sense.clone());
                    ActionResult::Sense(sense)
                }
            },
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nrows = self.grid.nrows();
        let ncols = self.grid.ncols();

        let header = (0..ncols)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("     ");

        f.write_str("    ")?;
        f.write_str(&header)?;
        f.write_char('\n')?;

        for i in 0..nrows {
            f.write_str(&format!("{}  ", i))?;
            for j in 0..ncols {
                let room = self.grid.room_at(&Pos::new(i, j));
                match room.is_visited() {
                    false => {
                        f.write_str("....  ")?;
                    }
                    true => {
                        let agent_pos = self.agent_position();
                        f.write_fmt(format_args!(
                            "{}{}{}{}  ",
                            room.get_kind(),
                            match agent_pos.row == i && agent_pos.col == j {
                                true => "A",
                                false => "_",
                            },
                            match room.has_sense(Sense::Breeze) {
                                true => "b",
                                false => "_",
                            },
                            match room.has_sense(Sense::Stench) {
                                true => "s",
                                false => "_",
                            },
                        ))?;
                    }
                }
            }

            if i < nrows - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
