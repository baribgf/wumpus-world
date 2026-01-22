use crate::agent::{Action, Agent, Observation};
use crate::kb::KnowledgeBase;
use crate::logic::Statement;

// Knowledge-Based Agent Impl //////////////////////////////
pub struct KnowledgeBasedAgent {
    kb: KnowledgeBase,
}

impl KnowledgeBasedAgent {
    pub fn new() -> Self {
        KnowledgeBasedAgent {
            kb: KnowledgeBase::new(),
        }
    }

    pub fn make_percept_stmt(&self, obs: &Observation) -> Statement {
        todo!()
    }

    pub fn make_action_stmt(&self, action: &Action) -> Statement {
        todo!()
    }

    pub fn ask_for_action(&self) -> Action {
        todo!()
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
