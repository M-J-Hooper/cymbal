use crate::{expr::*, literal::*, Error, ErrorKind, Result};
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
            Op::Add => "+",
            Op::Mul => ".",
        };

        write!(f, "({} {} {})", self.left, delimeter, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power() {
        assert_eq!("(x)^(2)", (Expr::Var('x') ^ new_integer(2)).to_string());
    }

    #[test]
    fn fractional_power() {
        assert_eq!("(x)^(1/2)", (Expr::Var('x') ^ new_number(1, 2)).to_string());
    }

    #[test]
    fn polynomial() {
        assert_eq!(
            "(((x)^(2) + x) + 3)",
            ((Expr::Var('x') ^ new_integer(2)) + 'x' + new_integer(3)).to_string()
        );
    }

    #[test]
    fn simple_multiplication() {
        assert_eq!(
            "((3 . x) . y)",
            (Expr::Lit(new_integer(3)) * 'x' * 'y').to_string()
        );
    }
}
