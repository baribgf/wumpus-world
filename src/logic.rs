use std::fmt::Display;
use std::hash::Hash;

type Left = Box<Statement>;
type Right = Box<Statement>;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Statement {
    Atomic(String),
    AndClause(Left, Right),
    OrClause(Left, Right),
    NotClause(Box<Statement>),
    ImplyClause(Left, Right),
    EquivalClause(Left, Right),
}

impl Statement {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn negate(self) -> Self {
        Self::NotClause(self.boxed())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Atomic(symbol) => f.write_str(&symbol),
            Statement::AndClause(left, right) => {
                let mut right_repr = format!("{}", right);
                if let Statement::AndClause(_, _) = &**right {
                    right_repr.remove(0);
                    right_repr.pop();
                }
                f.write_fmt(format_args!("({} & {})", left, right_repr))
            }
            Statement::OrClause(left, right) => {
                let mut right_repr = format!("{}", right);
                if let Statement::OrClause(_, _) = &**right {
                    right_repr.remove(0);
                    right_repr.pop();
                }
                f.write_fmt(format_args!("({} | {})", left, right_repr))
            }
            Statement::NotClause(stmt) => f.write_fmt(format_args!("~{}", stmt)),
            Statement::ImplyClause(left, right) => {
                f.write_fmt(format_args!("({} ⟹ {})", left, right))
            }
            Statement::EquivalClause(left, right) => {
                f.write_fmt(format_args!("({} ⟺ {})", left, right))
            }
        }
    }
}
