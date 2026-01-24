use std::{collections::HashSet, fmt::Display};

use crate::logic::Statement;

/// A knowledge base (`KB`) is defined to be a set of logical
/// statements, representing facts that an agent `“knows”`.
///
/// The `KnowledgeBase` struct is used for storing and querying
/// facts. It, provides an interface for storing facts (via `tell`)
/// and querying them (via `ask`).
pub struct KnowledgeBase {
    facts: HashSet<Statement>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        KnowledgeBase {
            facts: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.facts.clear();
    }

    /// Stores a fact into the knowledge base.
    pub fn tell(&mut self, stmt: Statement) {
        self.facts.insert(stmt);
    }

    /// Queries the knowledge base for a fact.
    pub fn ask(&self, stmt: &Statement) -> bool {
        /* Here, begins the real adventure of
        intelligent reasoning! TO BE CONTINUED.. */
        false
    }
}

impl Display for KnowledgeBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fact in &self.facts {
            f.write_fmt(format_args!("{}\n", fact)).unwrap()
        }
        Ok(())
    }
}
