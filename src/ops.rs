use crate::{expr::*, literal::*};

impl<T: Into<Expr>> std::ops::Add<T> for Expr {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Expr::Group(Group::add(self, other.into()))
    }
}

impl<T: Into<Expr>> std::ops::Mul<T> for Expr {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Expr::Group(Group::multiply(self, other.into()))
    }
}

impl<T: Into<Expr>> std::ops::Div<T> for Expr {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let pow = Expr::Pow(Power::new(other.into(), new_integer(-1)));
        Expr::Group(Group::multiply(self, pow))
    }
}

// ^ operator resembles exponentiation notation
impl<N: Into<Num>> std::ops::BitXor<N> for Expr {
    type Output = Self;

    fn bitxor(self, n: N) -> Self::Output {
        Expr::Pow(Power::new(self, n.into()))
    }
}

mod tests {
    use super::*;

    #[test]
    fn simple_multiply() {
        assert_eq!(
            "x . y . z".to_string(),
            (Expr::Var('x') * Expr::Var('y') * Expr::Var('z')).to_string()
        );
    }
}
