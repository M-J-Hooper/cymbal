use crate::{
    expr::*,
    literal::*,
};

impl std::ops::Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Expr::Lit(l), Expr::Lit(r)) => Expr::Lit(l + r),
            (Expr::Group(l), Expr::Group(r)) => Expr::Group(l + r),
            (Expr::Group(l), r) => Expr::Group(l + Group::add(vec![r])),
            (l, Expr::Group(r)) => Expr::Group(r + Group::add(vec![l])),
            (l, r) => Expr::Group(Group::add(vec![l, r])),
        }
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Expr::Lit(l), Expr::Lit(r)) => Expr::Lit(l * r),
            (Expr::Group(l), Expr::Group(r)) => Expr::Group(l * r),
            (Expr::Group(l), r) => Expr::Group(l * Group::multiply(vec![r])),
            (l, Expr::Group(r)) => Expr::Group(r * Group::multiply(vec![l])),
            (l, r) => Expr::Group(Group::multiply(vec![l, r])),
        }
    }
}

impl std::ops::Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let pow = Expr::Pow(Power::new(other, new_integer(-1)));
        Expr::Group(Group::multiply(vec![self, pow]))
    }
}

// ^ operator resembles exponentiation notation
impl std::ops::BitXor<Real> for Expr {
    type Output = Self;

    fn bitxor(self, n: Real) -> Self::Output {
        Expr::Pow(Power::new(self, n))
    }
}

impl std::ops::Add for Group {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self {
        match (&self.op, &other.op) {
            (Op::Add, Op::Add) => Group::add([&self.members[..], &other.members[..]].concat()),
            (Op::Add, Op::Mul) => {
                self.members.push(Expr::Group(other));
                self
            },
            (Op::Mul, Op::Add) => {
                other.members.push(Expr::Group(self));
                other
            },
            (Op::Mul, Op::Mul) => Group::add(vec![Expr::Group(self), Expr::Group(other)]),
        }
    }
}

impl std::ops::Mul for Group {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (&self.op, &other.op) {
            (Op::Mul, Op::Mul) => Group::multiply([&self.members[..], &other.members[..]].concat()),
            (Op::Add, Op::Mul) => expand(other, self),
            (Op::Mul, Op::Add) => expand(self, other),
            (Op::Add, Op::Add) => {
                let mut members = Vec::new();
                for l in self.members {
                    let m = Group::multiply(vec![l]) * other.clone();
                    members.extend(m.members);
                }
                Group::add(members)
            },
        }
    }
}

fn expand(mul: Group, add: Group) -> Group {
    let mut members = Vec::new();
    for r in add.members {
        let mut l = mul.clone();
        l.members.push(r);
        members.push(Expr::Group(l));
    }
    Group::add(members)
}

mod tests {
    use super::*;

    #[test]
    fn simple_multiply() {
        assert_eq!(
            "(x . y . z)".to_string(), 
            ((Expr::Var('x') * Expr::Var('y')) * Expr::Var('z')).to_string()
        );
    }

    #[test]
    fn associative_simple_multiply() {
        assert_eq!(
            "(y . z . x)".to_string(), 
            (Expr::Var('x') * (Expr::Var('y') * Expr::Var('z'))).to_string()
        );
    }

    #[test]
    fn scale_polynomial() {
        assert_eq!(
            "((x . y) + (x . z))".to_string(), 
            (Expr::Var('x') * (Expr::Var('y') + Expr::Var('z'))).to_string()
        );
    }

    #[test]
    fn expand() {
        assert_eq!(
            "((x . x) + (x . z) + (y . x) + (y . z))".to_string(), 
            ((Expr::Var('x') + Expr::Var('y')) * (Expr::Var('x') + Expr::Var('z'))).to_string()
        );
    }
}