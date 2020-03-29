use num::{
    rational,
    complex,
};
use num_bigint;

pub type Real = rational::BigRational;
pub type Complex = complex::Complex<Real>;

pub fn new_integer(n: isize) -> Real {
    let int = num_bigint::BigInt::from(n);
    Real::from(int)
}

pub fn new_rational(numer: isize, denom: isize) -> Real {
    let a = num_bigint::BigInt::from(numer);
    let b = num_bigint::BigInt::from(denom);
    rational::BigRational::new(a, b)
}

pub fn new_complex(re: isize, im: isize) -> Complex {
    complex::Complex::new(
        new_rational(re, 1), 
        new_rational(im, 1)
    )
}