use std::fmt::{Display, Write};

use crate::{
    agent::{Action, Direction, Observation, Sense},
    grid::{Grid, Pos},
    room::{Room, RoomKind, RoomSense},
};

const ARROW_PENALTY: usize = 10;

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

impl Environment {
    pub fn new() -> Self {
        const START_POS_ROW: usize = 3;
        const START_POS_COL: usize = 0;

        let mut grid = Grid::new(4, 4);

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

        let mut env = Environment {
            grid,
            score: 0,
            init_pos: Pos::new(START_POS_ROW, START_POS_COL),
            agent_pos: Pos::new(0, 0),
            curr_obs: Observation::new(Pos::new(0, 0), Sense::None),
        };

        Self::initialize(&mut env);

        env
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
        self.curr_obs.set_sense(Sense::None);
        self.grid.initialize();
        self.lightup_agent_position();
    }

    pub fn step(&mut self, action: &Action) -> ActionResult {
        match action {
            Action::Move(direction) => {
                if !self.is_direction_valid(&direction) {
                    return ActionResult::Sense(Sense::Bump);
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
                match self.current_room().get_kind() {
                    RoomKind::Pit => ActionResult::GameOver,
                    RoomKind::Wumpus => ActionResult::GameOver,
                    _ => ActionResult::Ok,
                }
            }
            Action::Shoot(direction) => {
                if !self.is_direction_valid(&direction) {
                    return ActionResult::Sense(Sense::Bump);
                }

                let target_position = self.agent_position() + &direction;
                match self.grid.room_at(&target_position).get_kind() {
                    RoomKind::Wumpus => {
                        self.grid
                            .mut_room_at(&target_position)
                            .set_kind(RoomKind::Void);
                        self.grid.mut_room_at(&target_position).set_visited(true);
                        self.set_score(self.score() - ARROW_PENALTY as isize);
                        return ActionResult::Sense(Sense::Scream);
                    }
                    _ => ActionResult::Ok,
                }
            }
            Action::Climb => match self.agent_pos == self.init_pos {
                true => ActionResult::GameOver,
                false => return ActionResult::Sense(Sense::Ceil),
            },
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        let nrows = self.grid.nrows();
        'outer: for i in 0..nrows {
            for j in 0..self.grid.ncols() {
                let room = self.grid.room_at(&Pos::new(i, j));
                match room.is_visited() {
                    false => {
                        res = f.write_str("....  ");
                    }
                    true => {
                        let agent_pos = self.agent_position();
                        res = f.write_fmt(format_args!(
                            "{}{}{}{}  ",
                            room.get_kind(),
                            match agent_pos.row == i && agent_pos.col == j {
                                true => "A",
                                false => "_",
                            },
                            match room.has_sense(RoomSense::Breeze) {
                                true => "b",
                                false => "_",
                            },
                            match room.has_sense(RoomSense::Stench) {
                                true => "s",
                                false => "_",
                            },
                        ));
                    }
                }

                if res.is_err() {
                    break 'outer;
                }
            }

            if i < nrows - 1 {
                res = f.write_char('\n');
            }
        }
        res
    }
}
