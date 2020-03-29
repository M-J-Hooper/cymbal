use crate::literal::*;

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

pub struct Power {
    pub n: Real, // FIXME: expression
    pub expr: Box<Expr>,
}

impl Power {
    pub fn new(n: Real, expr: Expr) -> Self {
        Power {
            n, 
            expr: Box::new(expr)
        }
    }
}

pub struct Group {
    pub op: Op,
    pub members: Vec<Expr>,
}

pub enum Op {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    #[test]
    fn power() {
        assert_eq!(2 + 2, 4);
    }
}