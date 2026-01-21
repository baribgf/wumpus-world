use std::fmt::Display;
use std::hash::Hash;

// Logic Primitives /////////////////////////////////////////
#[derive(Eq, Hash, PartialEq)]
pub struct Atomic {
    value: bool,
    name: String,
}

impl Atomic {
    pub fn new(name: &str, value: bool) -> Self {
        Atomic {
            name: name.to_string(),
            value,
        }
    }

    pub fn assign(&mut self, value: bool) {
        self.value = value;
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

#[derive(Eq, Hash, PartialEq)]
pub enum Statement {
    Atomic(Atomic),
    AndClause(Left, Right),
    OrClause(Left, Right),
    NotClause(Box<Statement>),
    ImplyClause(Left, Right),
}

impl Statement {
    fn eval(&self) -> bool {
        match self {
            Statement::Atomic(atomic) => atomic.value,
            Statement::AndClause(left, right) => left.eval() && right.eval(),
            Statement::OrClause(left, right) => left.eval() || right.eval(),
            Statement::NotClause(stmt) => !stmt.eval(),
            Statement::ImplyClause(left, right) => !left.eval() | right.eval(),
        }
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Atomic(atomic) => f.write_str(&atomic.name),
            Statement::AndClause(left, right) => {
                f.write_fmt(format_args!("({} & {})", left, right))
            }
            Statement::OrClause(left, right) => f.write_fmt(format_args!("({} | {})", left, right)),
            Statement::NotClause(stmt) => f.write_fmt(format_args!("~{}", stmt)),
            Statement::ImplyClause(left, right) => {
                f.write_fmt(format_args!("({} -> {})", left, right))
            }
        }
    }
}
