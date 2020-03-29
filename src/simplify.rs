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
            Expr::Var(_) => Ok(self),
            Expr::Lit(_) => Ok(self),
            Expr::Pow(p) => p.simplify(),
            Expr::Group(g) => g.simplify(),
        }
    }
}

impl Simplify for Power {
    fn simplify(self) -> Result<Expr> {
        if self.n == num::One::one() {
            return Ok(self.expr.simplify()?);
        } else if self.n == num::Zero::zero() {
            return Ok(Expr::Lit(num::One::one()));
        }

        match *self.expr {
            Expr::Lit(lit) => {
                let mut base = lit;
                let n = if self.n.is_negative() {
                    base = base.inv();
                    self.n.abs()
                } else {
                    self.n
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
            Expr::Pow(p) => {
                let mut expr = p.simplify()?;
                expr = if let Expr::Pow(inner) = expr {
                    Expr::Pow(Power::new(*inner.expr, self.n * inner.n))
                } else {
                    expr
                };
                Ok(expr)
            }
            _ => Ok(Expr::Pow(self)),
        }
    }
}

impl Simplify for Group {
    fn simplify(self) -> Result<Expr> {
        if self.members.is_empty() {
            return Ok(Expr::Lit(num::Zero::zero()));
        } else if self.members.len() == 1 {
            unimplemented!(); // return Ok(self.members[0]);
        }

        unimplemented!();
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