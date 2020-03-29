use crate::{
    expr::*,
    literal::*,
    Result,
    Error,
    ErrorKind,
};
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(v) => v.fmt(f),
            Expr::Lit(n) => n.fmt(f),
            Expr::Pow(p) => p.fmt(f),
            Expr::Group(g) => g.fmt(f),
        }
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})^({})", self.expr, self.n)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delimeter = match self.op {
            Op::Add => " + ",
            Op::Mul => " . "
        };
        
        let s = self.members.iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(delimeter);

        write!(f, "({})", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power() {
        assert_eq!("(x)^(2)", Power::new(Expr::Var('x'), new_integer(2)).to_string());
    }

    #[test]
    fn fractional_power() {
        assert_eq!("(x)^(1/2)", Power::new(Expr::Var('x'), new_rational(1, 2)).to_string());
    }

    #[test]
    fn polynomial() {
        assert_eq!(
            "((x)^(2) + x + 3+0i)", 
            Expr::Group(Group {
                op: Op::Add,
                members: vec![
                    Expr::Pow(Power::new(Expr::Var('x'), new_integer(2))),
                    Expr::Var('x'),
                    Expr::Lit(new_complex(3, 0)),
                ],
            }).to_string()
        );
    }

    #[test]
    fn simple_multiplication() {
        assert_eq!(
            "(3-2i . x . y)", 
            Expr::Group(Group {
                op: Op::Mul,
                members: vec![
                    Expr::Lit(new_complex(3, -2)),
                    Expr::Var('x'),
                    Expr::Var('y'),
                ],
            }).to_string()
        );
    }
}