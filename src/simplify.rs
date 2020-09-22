use crate::{expr::*, literal::*, Error, ErrorKind, Result};
use num::{rational::Ratio, traits::One, traits::Signed, traits::Zero};
use num_traits::cast::ToPrimitive;

pub trait Simplify {
    fn simplify(self) -> Result<Expr>;
}

impl Simplify for Expr {
    fn simplify(self) -> Result<Expr> {
        match self {
            Expr::Pow(p) => p.simplify(),
            Expr::Group(g) => g.simplify(),
            _ => Ok(self),
        }
    }
}

impl Simplify for Power {
    fn simplify(self) -> Result<Expr> {
        if self.n == num::Zero::zero() {
            return Ok(Expr::Lit(num::One::one()));
        }

        let inner = self.expr.simplify()?;
        if self.n == num::One::one() {
            return Ok(inner);
        }

        match inner {
            Expr::Lit(lit) => simplify_power_of_literal(lit, self.n),
            Expr::Pow(p) => Ok(Expr::Pow(Power::new(*p.expr, self.n * p.n))),
            _ => Ok(Expr::Pow(Power::new(inner, self.n))),
        }
    }
}

fn simplify_power_of_literal(lit: Num, n: Num) -> Result<Expr> {
    let mut base = lit;
    let n = if n.is_negative() {
        base = base.recip();
        n.abs()
    } else {
        n
    };

    let exp = n.numer().to_usize().ok_or(Error {
        kind: ErrorKind::TooBig,
        msg: "Exponent too large to apply".into(),
    })?;

    let res = num::pow(base, exp);
    if n.is_integer() {
        Ok(Expr::Lit(res))
    } else {
        let new_exp: Num = Ratio::from(n.denom().to_owned()).recip();
        Ok(Expr::Pow(Power::new(Expr::Lit(res), new_exp)))
    }
}

impl Simplify for Group {
    fn simplify(self) -> Result<Expr> {
        let l = self.left.simplify()?;
        let r = self.right.simplify()?;

        let expr = match (self.op, l, r) {
            (Op::Add, Expr::Lit(n), expr) | (Op::Add, expr, Expr::Lit(n)) if n.is_zero() => expr,
            (Op::Mul, Expr::Lit(n), expr) | (Op::Mul, expr, Expr::Lit(n)) if n.is_one() => expr,
            (Op::Add, left, right) => Expr::Group(Group::add(left, right)),
            (Op::Mul, left, right) => Expr::Group(Group::multiply(left, right)),
        };
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_power() {
        assert_eq!(
            Expr::Lit(new_integer(8)),
            Power::new(Expr::Lit(new_integer(2).into()), new_integer(3))
                .simplify()
                .unwrap()
        );
    }

    #[test]
    fn negative_power() {
        assert_eq!(
            Expr::Lit(new_number(1, 8).into()),
            Power::new(Expr::Lit(new_integer(2).into()), new_integer(-3))
                .simplify()
                .unwrap()
        );
    }

    #[test]
    fn fractional_power() {
        assert_eq!(
            Expr::Pow(Power::new(
                Expr::Lit(new_integer(8).into()),
                new_number(1, 2)
            )),
            Power::new(Expr::Lit(new_integer(2).into()), new_number(3, 2))
                .simplify()
                .unwrap()
        );
    }

    #[test]
    fn power_of_one() {
        assert_eq!(
            Expr::Var('x'),
            Power::new(Expr::Var('x'), new_integer(1))
                .simplify()
                .unwrap()
        );
    }

    #[test]
    fn power_of_zero() {
        assert_eq!(
            Expr::Lit(num::One::one()),
            Power::new(Expr::Var('x'), new_integer(0))
                .simplify()
                .unwrap()
        );
    }

    #[test]
    fn nested_powers() {
        assert_eq!(
            Expr::Pow(Power::new(Expr::Var('x'), new_number(-1, 4))),
            Expr::Pow(Power::new(
                Expr::Pow(Power::new(Expr::Var('x'), new_number(2, 3))),
                new_number(3, -8),
            ))
            .simplify()
            .unwrap()
        );
    }
}
