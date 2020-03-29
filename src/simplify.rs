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
                    msg: "Exponent to big".into(),
                })?;

                let res = num::pow(base, exp);
                if n.is_integer() {
                    Ok(Expr::Lit(res))
                } else {
                    let new_exp: Real = Ratio::from(n.denom().to_owned()).recip();
                    Ok(Expr::Pow(Power::new(new_exp, Expr::Lit(res))))
                }
            }
            Expr::Pow(p) => {
                let n = p.n.clone();
                let mut expr = p.simplify()?;
                expr = if let Expr::Pow(inner) = expr {
                    Expr::Pow(Power::new(n * inner.n, *inner.expr))
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
    
}