use crate::agent::{Action, Agent, Observation};
use crate::kb::KnowledgeBase;
use crate::logic::Statement;

// Knowledge-Based Agent Impl //////////////////////////////
pub struct KnowledgeBasedAgent {
    kb: KnowledgeBase,
    start_pos: Pos,
    grid_rows: usize,
    grid_cols: usize,
}

impl KnowledgeBasedAgent {
    pub fn new(start_pos: &Pos, grid_rows: usize, grid_cols: usize) -> Self {
        let mut agent = Self {
            kb: KnowledgeBase::new(),
            start_pos: start_pos.clone(),
            grid_rows,
            grid_cols,
        };
        agent.reset();

        agent
    }

    pub fn make_percept_stmt(&self, obs: &Observation) -> Statement {
        todo!()
    }

    pub fn make_action_stmt(&self, action: &Action) -> Statement {
        todo!()
    }

    pub fn reset(&mut self) {
        self.kb.clear();

        // Start position contains no Pit and no Wumpus
        let (row, col) = (self.start_pos.row, self.start_pos.col);
        let p = Atomic::new(&format!("P_{},{}", row, col));
        let w = Atomic::new(&format!("W_{},{}", row, col));
        let stmt = Statement::AndClause(
            Statement::NotClause(Statement::Atomic(p).boxed()).boxed(),
            Statement::NotClause(Statement::Atomic(w).boxed()).boxed(),
        );

        self.kb.tell(stmt);

        // There's only one Wumpus in the grid
        let mut mutuals: Vec<Statement> = Vec::new();
        for i in 0..self.grid_rows {
            for j in 0..self.grid_cols {
                if Pos::new(i, j) == self.start_pos {
                    continue;
                }

                mutuals.push(Statement::Atomic(Atomic::new(&format!("W_{},{}", i, j))));
            }
        }

        self.kb.tell(make_mutuals(&mutuals).unwrap());
    }
}

impl Agent for KnowledgeBasedAgent {
    fn act(&mut self, obs: &Observation) -> Action {
        // First, transform observation into a statement,
        // and `tell` it to the `KB`
        self.kb.tell(self.make_percept_stmt(obs));

        // Then, `ask` the `KB` for an action
        let action = self.ask_for_action();

        // Finally, `tell` the `KB` for the chosen action
        self.kb.tell(self.make_action_stmt(&action));
        action
    }
}
////////////////////////////////////////////////////////////
