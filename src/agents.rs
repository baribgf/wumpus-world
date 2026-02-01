use std::collections::HashSet;

use crate::agent::{Action, Agent, Direction, Observation};
use crate::env::Sense;
use crate::grid::Pos;
use crate::kb::KnowledgeBase;
use crate::logic::Statement;

struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn of<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            vec: iter.into_iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn peek(&self) -> &T {
        self.vec.last().unwrap()
    }

    pub fn mut_peek(&mut self) -> &mut T {
        self.vec.last_mut().unwrap()
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.vec.pop().unwrap()
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }
}

type Frame<T> = Stack<T>;

// Knowledge-Based Agent Impl //////////////////////////////
pub struct KnowledgeBasedAgent {
    kb: KnowledgeBase,
    start_pos: Pos,
    curr_pos: Pos,
    grid_rows: usize,
    grid_cols: usize,
    stack: Stack<Frame<Direction>>,
    visited: HashSet<Pos>,
    satisfied: bool,
    total_treasures: usize,
    treasures: usize,
}

impl KnowledgeBasedAgent {
    pub fn new(start_pos: &Pos, grid_rows: usize, grid_cols: usize) -> Self {
        let mut agent = Self {
            kb: KnowledgeBase::new(),
            start_pos: start_pos.clone(),
            curr_pos: Pos::new(0, 0),
            grid_rows,
            grid_cols,
            stack: Stack::new(),
            visited: HashSet::new(),
            satisfied: false,
            total_treasures: 1,
            treasures: 0,
        };
        agent.reset();

        agent
    }

    pub fn make_percept_stmt(&self, obs: &Observation) -> Statement {
        let pos = obs.position();
        let mut senses = obs.senses().clone();

        if senses.len() == 0 {
            /* current cell contains no sense */
            return make_void_atomic(pos);
        }

        let mut conjuncts: Vec<Statement> = Vec::new();
        if senses.contains(&Sense::Stench) {
            senses.remove(&Sense::Stench);
            conjuncts.push(make_stench_atomic(pos));
        } else {
            conjuncts.push(make_stench_atomic(pos).negate());
        }

        if senses.contains(&Sense::Breeze) {
            senses.remove(&Sense::Breeze);
            conjuncts.push(make_breeze_atomic(pos));
        } else {
            conjuncts.push(make_breeze_atomic(pos).negate());
        }

        for sense in senses {
            conjuncts.push(match sense {
                Sense::Scream(dir) => {
                    let wumpus_pos = pos + &dir;
                    Statement::Atomic(format!("Sc_{},{}", wumpus_pos.row, wumpus_pos.col))
                }
                Sense::Glitter => Statement::Atomic(format!("Gl_{},{}", pos.row, pos.col)),
                _ => panic!(),
            });
        }

        make_conjuncts(&conjuncts).unwrap()
    }

    /* pub fn make_action_stmt(&self, action: &Action) -> Statement {
        todo!()
    } */

    fn is_direction_valid(&self, pos: &Pos, direction: &Direction) -> bool {
        match direction {
            Direction::North => pos.row > 0,
            Direction::South => pos.row < self.grid_rows - 1,
            Direction::East => pos.col < self.grid_cols - 1,
            Direction::West => pos.col > 0,
        }
    }

    fn neighborhood(&self, pos: &Pos) -> HashSet<Pos> {
        let mut dirs = HashSet::new();
        dirs.insert(Direction::North);
        dirs.insert(Direction::South);
        dirs.insert(Direction::East);
        dirs.insert(Direction::West);

        dirs.retain(|dir| self.is_direction_valid(pos, dir));

        dirs.iter().map(|dir| pos + dir).collect()
    }

    fn retain_non_visited(&self, directions: &mut HashSet<Direction>) {
        directions.retain(|dir| !self.visited.contains(&(&self.curr_pos + dir)));
    }

    /// The agent part that actually uses reasoning to eliminate
    /// non-safe directions according to its logic. Given a set
    /// of directions, it retains safe ones by asking the `KB`
    /// for their safety.
    fn retain_safe(&mut self, directions: &mut HashSet<Direction>) {
        let positions: HashSet<(Direction, Pos)> = directions
            .iter()
            .cloned()
            .map(|dir| (dir.clone(), &self.curr_pos + &dir))
            .collect();

        for (dir, pos) in positions {
            let answer = self.kb.ask(&make_safe_atomic(&pos));
            if answer.is_none() || !answer.unwrap() {
                directions.remove(&dir);
            }
        }
    }

    /// Updates the agent's current position by moving it in the
    /// given direction and marks the new position as visited.
    ///
    /// ## Arguments
    ///
    /// * `direction` - The direction in which the agent should move
    ///
    /// ## Side Effects
    ///
    /// * Updates `self.curr_pos` to the new position
    /// * Inserts the new position into `self.visited` set
    fn update_and_mark_position(&mut self, direction: &Direction) {
        self.curr_pos += direction;
        self.visited.insert(self.curr_pos.clone());
    }

    pub fn ask_for_action(&mut self, obs: &Observation) -> Action {
        /* Action strategy algorithm */

        // ---------------------------------------
        // =*= The Explore-Backtrack Algorithm =*=
        // ---------------------------------------

        if obs.senses().contains(&Sense::Glitter) {
            self.treasures += 1;
            return Action::Grab;
        }

        if self.treasures == self.total_treasures {
            self.satisfied = true;
        }

        if self.satisfied && self.curr_pos == self.start_pos {
            return Action::Climb;
        }

        let frame = self.stack.peek();
        if frame.is_empty() {
            self.stack.pop();
            let mut directions = obs.directions().clone();
            self.retain_non_visited(&mut directions);
            self.retain_safe(&mut directions);
            if directions.is_empty() {
                if self.stack.is_empty() {
                    // There's no way to backtrack, so climb out!
                    return Action::Climb;
                }
                let frame = self.stack.mut_peek();
                let dir = frame.pop();
                let rev = dir.reverse();
                self.update_and_mark_position(&rev);
                return Action::Move(rev);
            } else {
                let frame = Frame::of(directions.into_iter());
                let dir = frame.peek().clone();
                self.stack.push(frame);
                self.stack.push(Frame::new());
                self.update_and_mark_position(&dir);
                return Action::Move(dir);
            }
        } else {
            let dir = frame.peek().clone();
            self.stack.push(Frame::new());
            self.update_and_mark_position(&dir);
            return Action::Move(dir);
        }
    }

    fn axiomatize(&mut self) {
        // There's only one Wumpus in the grid
        let all = (0..self.grid_rows)
            .flat_map(|a| (0..self.grid_cols).map(move |b| (a, b)))
            .map(|(row, col)| make_wumpus_atomic(&Pos::new(row, col)));

        let disjuncts: Vec<Statement> = all.clone().collect();
        self.kb.tell(make_disjuncts(&disjuncts).unwrap());

        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let cause = make_wumpus_atomic(&Pos::new(i, j));
                let conjuncts: Vec<Statement> = all
                    .clone()
                    .map(|s| Statement::NotClause(s.boxed()))
                    .collect();

                self.kb.tell(Statement::ImplyClause(
                    cause.boxed(),
                    make_conjuncts(&conjuncts).unwrap().boxed(),
                ));
            }
        }

        // A safe place is one which contains no Pit and no Wumpus
        // S_r,c <=> (~W_r,c & ~P_r,c)
        let mut conjuncts: Vec<Statement> = Vec::new();
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                conjuncts.push(Statement::EquivalClause(
                    make_safe_atomic(&Pos::new(i, j)).boxed(),
                    Statement::AndClause(
                        Statement::NotClause(make_wumpus_atomic(&Pos::new(i, j)).boxed()).boxed(),
                        Statement::NotClause(make_pit_atomic(&Pos::new(i, j)).boxed()).boxed(),
                    )
                    .boxed(),
                ));
            }
        }
        self.kb.tell(make_conjuncts(&conjuncts).unwrap());

        // Start position is safe
        self.kb.tell(make_safe_atomic(&self.start_pos));

        // A void position is safe and all neighboring positions
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let void_pos = Pos::new(i, j);
                let void_stmt = make_void_atomic(&void_pos);
                let mut conjuncts: Vec<Statement> = Vec::new();
                conjuncts.push(make_safe_atomic(&void_pos));
                conjuncts.extend(
                    self.neighborhood(&void_pos)
                        .iter()
                        .map(|np| make_safe_atomic(&np)),
                );
                self.kb.tell(Statement::EquivalClause(
                    void_stmt.boxed(),
                    make_conjuncts(&conjuncts).unwrap().boxed(),
                ));
            }
        }

        // No position can contain both Wumpus and Pit, that is,
        // all positions satisfy: ~(W_r,c & P_r,c)
        let mut conjuncts: Vec<Statement> = Vec::new();
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let pos = Pos::new(i, j);
                conjuncts.push(Statement::NotClause(
                    Statement::AndClause(
                        make_wumpus_atomic(&pos).boxed(),
                        make_pit_atomic(&pos).boxed(),
                    )
                    .boxed(),
                ));
            }
        }
        self.kb.tell(make_conjuncts(&conjuncts).unwrap());

        // A position is breezy iff some neighboring
        // position contains a Pit
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let pos = Pos::new(i, j);
                let disjuncts: Vec<Statement> = self
                    .neighborhood(&pos)
                    .iter()
                    .map(|np| make_pit_atomic(&np))
                    .collect();
                self.kb.tell(Statement::EquivalClause(
                    make_breeze_atomic(&pos).boxed(),
                    make_disjuncts(&disjuncts).unwrap().boxed(),
                ));
            }
        }

        // A position is stenchy iff some neighboring
        // position contains a Wumpus
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let pos = Pos::new(i, j);
                let disjuncts: Vec<Statement> = self
                    .neighborhood(&pos)
                    .iter()
                    .map(|np| make_wumpus_atomic(&np))
                    .collect();

                self.kb.tell(Statement::EquivalClause(
                    make_stench_atomic(&pos).boxed(),
                    make_disjuncts(&disjuncts).unwrap().boxed(),
                ));
            }
        }

        // A position containing a Pit implies all neighboring
        // positions to be breezy
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let pos = Pos::new(i, j);
                let conjuncts: Vec<Statement> = self
                    .neighborhood(&pos)
                    .iter()
                    .map(|np| make_breeze_atomic(&np))
                    .collect();

                self.kb.tell(Statement::ImplyClause(
                    make_pit_atomic(&pos).boxed(),
                    make_conjuncts(&conjuncts).unwrap().boxed(),
                ));
            }
        }

        // A position containing a Wumpus implies all neighboring
        // positions to be stenchy
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                let pos = Pos::new(i, j);
                let conjuncts: Vec<Statement> = self
                    .neighborhood(&pos)
                    .iter()
                    .map(|np| make_stench_atomic(&np))
                    .collect();

                self.kb.tell(Statement::ImplyClause(
                    make_wumpus_atomic(&pos).boxed(),
                    make_conjuncts(&conjuncts).unwrap().boxed(),
                ));
            }
        }
    }

    pub fn reset(&mut self) {
        self.treasures = 0;
        self.satisfied = false;
        self.kb.clear();
        self.visited.clear();
        self.visited.insert(self.start_pos.clone());
        self.stack.clear();
        self.stack.push(Frame::new());
        self.curr_pos = self.start_pos.clone();

        self.axiomatize();
    }
}

impl Agent for KnowledgeBasedAgent {
    fn act(&mut self, obs: &Observation) -> Action {
        // First, transform observation into a statement,
        // and `tell` it to the `KB`
        self.kb.tell(self.make_percept_stmt(obs));

        // Then, `ask` the `KB` for an action
        let action = self.ask_for_action(obs);

        // Finally, `tell` the `KB` for the chosen action
        // self.kb.tell(self.make_action_stmt(&action));
        action
    }
}
////////////////////////////////////////////////////////////

fn make_safe_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("S_{},{}", pos.row, pos.col))
}

fn make_void_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("V_{},{}", pos.row, pos.col))
}

fn make_wumpus_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("W_{},{}", pos.row, pos.col))
}

fn make_pit_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("P_{},{}", pos.row, pos.col))
}

fn make_breeze_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("Br_{},{}", pos.row, pos.col))
}

fn make_stench_atomic(pos: &Pos) -> Statement {
    Statement::Atomic(format!("St_{},{}", pos.row, pos.col))
}

fn make_disjuncts(stmts: &[Statement]) -> Option<Statement> {
    match stmts.first() {
        Some(stmt) => match make_disjuncts(&stmts[1..]) {
            Some(rhs) => Some(Statement::OrClause(stmt.clone().boxed(), rhs.boxed())),
            None => Some(stmt.to_owned()),
        },
        None => None,
    }
}

fn make_conjuncts(stmts: &[Statement]) -> Option<Statement> {
    match stmts.first() {
        Some(stmt) => match make_conjuncts(&stmts[1..]) {
            Some(rhs) => Some(Statement::AndClause(stmt.clone().boxed(), rhs.boxed())),
            None => Some(stmt.to_owned()),
        },
        None => None,
    }
}
