use crate::{
    expr::*,
    literal::*,
    Result,
    Error,
    ErrorKind,
};
use num_traits::cast::ToPrimitive;
use num::{
    traits::Signed,
    rational::Ratio,
};

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

fn simplify_power_of_literal(lit: Complex, n: Real) -> Result<Expr> {
    let mut base = lit;
    let n = if n.is_negative() {
        base = base.inv();
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
        let new_exp: Real = Ratio::from(n.denom().to_owned()).recip();
        Ok(Expr::Pow(Power::new(Expr::Lit(res), new_exp)))
    }
}

impl Simplify for Group {
    fn simplify(self) -> Result<Expr> {
        if self.members.is_empty() {
            return Ok(Expr::Lit(num::Zero::zero()));
        } else if self.members.len() == 1 {
            return Ok(self.members.into_iter().nth(0).unwrap());
        }

        let i = Expr::Group(Group::identity(&self.op));
        let f = match &self.op {
            Op::Add => <Expr as std::ops::Add>::add,
            Op::Mul => <Expr as std::ops::Mul>::mul,
        };

        let simplified = self.members.into_iter()
            .map(|m| m.simplify())
            .collect::<Result<Vec<_>>>()?;

        Ok(simplified.into_iter().fold(i, f))
        // FIXME collate common expressions together
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_power() {
        assert_eq!(
            Expr::Lit(new_complex(8, 0)),
            Power::new(Expr::Lit(new_integer(2).into()), new_integer(3)).simplify().unwrap()
        );
    }

    #[test]
    fn negative_power() {
        assert_eq!(
            Expr::Lit(new_rational(1, 8).into()),
            Power::new(Expr::Lit(new_integer(2).into()), new_integer(-3)).simplify().unwrap()
        );
    }

    #[test]
    fn fractional_power() {
        assert_eq!(
            Expr::Pow(Power::new(Expr::Lit(new_integer(8).into()), new_rational(1, 2))),
            Power::new(Expr::Lit(new_integer(2).into()), new_rational(3, 2)).simplify().unwrap()
        );
    }

    #[test]
    fn complex_squared() {
        assert_eq!(
            Expr::Lit(new_complex(-3, 4)),
            Power::new(Expr::Lit(new_complex(1, 2)), new_integer(2)).simplify().unwrap()
        );
    }

    #[test]
    fn power_of_one() {
        assert_eq!(
            Expr::Var('x'),
            Power::new(Expr::Var('x'), new_integer(1)).simplify().unwrap()
        );
    }

    #[test]
    fn power_of_zero() {
        assert_eq!(
            Expr::Lit(num::One::one()),
            Power::new(Expr::Var('x'), new_integer(0)).simplify().unwrap()
        );
    }

    #[test]
    fn nested_powers() {
        assert_eq!(
            Expr::Pow(Power::new(Expr::Var('x'), new_rational(-1, 4))),
            Expr::Pow(Power::new(
                Expr::Pow(Power::new(Expr::Var('x'), new_rational(2, 3))),
                new_rational(3, -8),
            )).simplify().unwrap()
        );
    }
}