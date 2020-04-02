use crate::literal::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Var(char),
    Lit(Complex),
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
    pub n: Real, // FIXME: expression
    pub expr: Box<Expr>,
}

impl Power {
    pub fn new(expr: Expr, n: Real) -> Self {
        Power { n, expr: Box::new(expr) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    pub op: Op,
    pub members: Vec<Expr>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn identity(&self) -> Complex {
        let r = match *self {
            Op::Add => num::Zero::zero(),
            Op::Mul => num::One::one(),
        };
        Complex::new(r, num::Zero::zero())
    }
}

impl Group {
    pub fn add(members: Vec<Expr>) -> Self {
        Group { op: Op::Add, members }
    }

    pub fn multiply(members: Vec<Expr>) -> Self {
        Group { op: Op::Mul, members }
    }

    pub fn identity(op: &Op) -> Self {
        let expr = Expr::Lit(op.identity());
        Group { op: op.clone(), members: vec![expr] }
    }
}

impl From<Complex> for Expr {
    fn from(c: Complex) -> Self { 
        Expr::Lit(c)
    }
}

impl From<char> for Expr {
    fn from(c: char) -> Self { 
        Expr::Var(c)
    }
}

impl From<Expr> for Group {
    fn from(expr: Expr) -> Self { 
        match expr {
            Expr::Group(g) => g,
            e => Group::multiply(vec![e]),
        }
    }
}