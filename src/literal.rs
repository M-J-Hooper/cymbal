use crate::expr::Expr;

use num::rational;
use num_bigint;

pub type Num = rational::BigRational;

pub fn new_integer(n: isize) -> Num {
    let int = num_bigint::BigInt::from(n);
    Num::from(int)
}

pub fn new_number(numer: isize, denom: isize) -> Num {
    let a = num_bigint::BigInt::from(numer);
    let b = num_bigint::BigInt::from(denom);
    rational::BigRational::new(a, b)
}

impl From<Num> for Expr {
    fn from(n: Num) -> Self {
        Expr::Lit(n)
    }
}
