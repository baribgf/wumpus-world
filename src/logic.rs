use std::fmt::Display;
use std::hash::Hash;

// Logic Primitives /////////////////////////////////////////
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Atomic {
    name: String,
}

impl Atomic {
    pub fn new(name: &str) -> Self {
        Atomic {
            name: name.to_string()
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct BinaryOp {
    left: Statement,
    right: Statement,
}

impl BinaryOp {
    pub fn new(left: Statement, right: Statement) -> Self {
        BinaryOp { left, right }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct UnaryOp {
    stmt: Statement,
}

impl UnaryOp {
    pub fn new(stmt: Statement) -> Self {
        UnaryOp { stmt }
    }
}
///////////////////////////////////////////////////////////

type Left = Box<Statement>;
type Right = Box<Statement>;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Statement {
    Atomic(Atomic),
    AndClause(Left, Right),
    OrClause(Left, Right),
    XorClause(Left, Right),
    NotClause(Box<Statement>),
    ImplyClause(Left, Right),
}

impl Statement {
    pub fn eval(&self) -> bool {
        match self {
            Self::Atomic(_) => true,
            Self::AndClause(left, right) => left.eval() & right.eval(),
            Self::OrClause(left, right) => left.eval() | right.eval(),
            Self::XorClause(left, right) => left.eval() ^ right.eval(),
            Self::NotClause(stmt) => !stmt.eval(),
            Self::ImplyClause(left, right) => !left.eval() | right.eval(),
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Atomic(atomic) => f.write_str(&atomic.name),
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
            },
            Statement::XorClause(left, right) => {
                let mut right_repr = format!("{}", right);
                if let Statement::XorClause(_, _) = &**right {
                    right_repr.remove(0);
                    right_repr.pop();
                }
                f.write_fmt(format_args!("({} ⊕  {})", left, right_repr))
            }
            Statement::NotClause(stmt) => f.write_fmt(format_args!("~{}", stmt)),
            Statement::ImplyClause(left, right) => {
                f.write_fmt(format_args!("({} -> {})", left, right))
            }
        }
    }
}
