pub mod display;
pub mod expr;
pub mod literal;
pub mod ops;
pub mod simplify;

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    TooBig,
}

pub type Result<T> = std::result::Result<T, Error>;
