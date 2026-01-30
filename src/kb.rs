use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

use crate::logic::Statement;

/// A knowledge base (`KB`) is defined to be a set of logical
/// statements, representing facts that an agent `“knows”`.
///
/// The `KnowledgeBase` struct is used for storing and querying
/// facts. It, provides an interface for storing facts (via `tell`)
/// and querying them (via `ask`).
pub struct KnowledgeBase {
    facts: HashSet<Statement>,
    pending: HashSet<Statement>,
    cache: HashMap<Statement, bool>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        KnowledgeBase {
            facts: HashSet::new(),
            pending: HashSet::new(),
            cache: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.facts.clear();
        self.pending.clear();
        self.invalidate_cache();
    }

    pub fn invalidate_cache(&mut self) {
        self.cache.clear();
    }

    /// Stores a fact into the knowledge base.
    pub fn tell(&mut self, stmt: Statement) {
        /* The following statement transformation pipeline unsures
        they take some form that is suitable for automatic theorem
        proving using the backward chaining algorithm */

        let new_facts: HashSet<Statement> = match stmt {
            Statement::AndClause(_, _) => get_conjuncts(&stmt).into(),
            _ => HashSet::from([stmt]),
        }
        .drain()
        .flat_map(|stmt| match stmt {
            Statement::NotClause(ref neg_stmt) => match neg_stmt.deref() {
                Statement::AndClause(left, right) => vec![
                    Statement::ImplyClause(
                        left.clone(),
                        Statement::NotClause(right.clone()).boxed(),
                    ),
                    Statement::ImplyClause(
                        right.clone(),
                        Statement::NotClause(left.clone()).boxed(),
                    ),
                ],
                _ => vec![stmt],
            },
            _ => vec![stmt],
        })
        .flat_map(|stmt| match stmt {
            Statement::EquivalClause(left, right) => vec![
                Statement::ImplyClause(left.clone(), right.clone()),
                Statement::ImplyClause(right.clone(), left.clone()),
            ],
            _ => vec![stmt],
        })
        .flat_map(|stmt| match stmt {
            Statement::ImplyClause(ref left, ref right) => match &right.deref() {
                Statement::AndClause(_, _) => get_conjuncts(right)
                    .iter()
                    .cloned()
                    .map(|conj| Statement::ImplyClause(left.clone(), conj.boxed()))
                    .collect(),
                _ => vec![stmt],
            },
            _ => vec![stmt],
        })
        .flat_map(|stmt| match stmt {
            Statement::ImplyClause(ref left, ref right) => {
                if matches!(**left, Statement::Atomic(_)) && matches!(**right, Statement::Atomic(_))
                {
                    vec![
                        stmt.clone(),
                        Statement::ImplyClause(
                            right.clone().negate().boxed(),
                            left.clone().negate().boxed(),
                        ),
                    ]
                } else {
                    vec![stmt]
                }
            }
            _ => vec![stmt],
        })
        .collect();

        self.facts.extend(new_facts);
        self.invalidate_cache();
    }

    /// Queries the knowledge base for a fact.
    pub fn ask(&mut self, stmt: &Statement) -> Option<bool> {
        /* Here, begins the real adventure of intelligent reasoning! */
        if self.pending.contains(stmt) {
            return None;
        }

        // println!("Asking truth for: {}", stmt);
        if self.cache.contains_key(stmt) {
            let cache_res = *self.cache.get(stmt).unwrap();
            /* match cache_res {
                true => println!("True: cache"),
                false => println!("False: cache"),
            } */
            return Some(cache_res);
        }

        self.pending.insert(stmt.clone());
        let result = {
            let mut found_early = false;
            let mut early_res = false;
            for fact in &self.facts {
                if *fact == *stmt {
                    found_early = true;
                    early_res = true;
                    break;
                } else if let Statement::NotClause(negated) = fact {
                    if **negated == *stmt {
                        found_early = true;
                        early_res = false;
                        break;
                    }
                }
            }

            if found_early {
                Some(early_res)
            } else {
                // And here, begins the glorious backward chainer !!
                let entailers: HashSet<Statement> = self
                    .facts
                    .iter()
                    .map(|fact| match fact {
                        Statement::ImplyClause(left, right) => {
                            if **right == *stmt {
                                Some(left.deref().clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .filter(|item| item.is_some())
                    .map(|item| item.unwrap())
                    .collect();

                let mut entail_res = None;
                'entail: for head in entailers {
                    match head {
                        Statement::Atomic(_) | Statement::NotClause(_) => {
                            let answer = self.ask(&head);
                            if answer.is_some() && answer.unwrap() {
                                entail_res = Some(true);
                                break 'entail;
                            }
                        }
                        Statement::AndClause(_, _) => {
                            for conj in get_conjuncts(&head) {
                                let answer = self.ask(&conj);
                                if answer.is_none() || !answer.unwrap() {
                                    entail_res = Some(false);
                                    continue 'entail;
                                }
                            }
                            entail_res = Some(true);
                            break;
                        }
                        Statement::OrClause(_, _) => {
                            for disj in get_disjuncts(&head) {
                                let answer = self.ask(&disj);
                                if answer.is_some() && answer.unwrap() {
                                    entail_res = Some(true);
                                    break 'entail;
                                }
                            }
                        }
                        _ => continue,
                    }
                }

                entail_res
            }
        };

        if result.is_some() {
            /* match result.unwrap() {
                true => println!("True"),
                false => println!("False"),
            } */
            self.cache.insert(stmt.clone(), result.unwrap());
        } else {
            // println!("Cannot tell!")
        }
        self.pending.remove(stmt);
        result
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

fn get_conjuncts(stmt: &Statement) -> HashSet<Statement> {
    let mut conjuncts = HashSet::new();
    let mut queue: Vec<&Statement> = Vec::new();
    queue.push(stmt);

    while !queue.is_empty() {
        let expandee = queue.pop().unwrap();
        match expandee {
            Statement::AndClause(le, re) => {
                queue.push(le);
                queue.push(re);
            }
            other => {
                conjuncts.insert(other.clone());
            }
        }
    }

    conjuncts
}

fn get_disjuncts(stmt: &Statement) -> HashSet<Statement> {
    let mut disjuncts = HashSet::new();
    let mut queue: Vec<&Statement> = Vec::new();
    queue.push(stmt);

    while !queue.is_empty() {
        let expandee = queue.pop().unwrap();
        match expandee {
            Statement::OrClause(le, re) => {
                queue.push(le);
                queue.push(re);
            }
            other => {
                disjuncts.insert(other.clone());
            }
        }
    }

    disjuncts
}
