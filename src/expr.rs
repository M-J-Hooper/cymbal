use crate::literal::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Var(char),
    Lit(Num),
    Pow(Power),
    Group(Group),
}

pub struct Statement {
    pub kind: StatementKind,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

pub enum StatementKind {
    Equality,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Power {
    pub n: Num, // FIXME: expression
    pub expr: Box<Expr>,
}

impl Power {
    pub fn new(expr: Expr, n: Num) -> Self {
        Power {
            n,
            expr: Box::new(expr),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    pub op: Op,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn identity(&self) -> Num {
        match *self {
            Op::Add => num::Zero::zero(),
            Op::Mul => num::One::one(),
        }
    }
}

impl Group {
    pub fn add(left: Expr, right: Expr) -> Self {
        Group {
            op: Op::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn multiply(left: Expr, right: Expr) -> Self {
        Group {
            op: Op::Mul,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn identity(op: &Op) -> Self {
        let i = Expr::Lit(op.identity());
        match op {
            Op::Add => Self::add(i.clone(), i),
            Op::Mul => Self::multiply(i.clone(), i),
        }
    }
}

impl From<char> for Expr {
    fn from(c: char) -> Self {
        Expr::Var(c)
    }
}

impl From<Group> for Expr {
    fn from(g: Group) -> Self {
        Expr::Group(g)
    }
}

impl From<Power> for Expr {
    fn from(p: Power) -> Self {
        Expr::Pow(p)
    }
}
