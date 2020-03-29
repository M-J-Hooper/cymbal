pub mod literal;
pub mod expr;
pub mod simplify;
mod display;

pub struct Error {
    kind: ErrorKind,
    msg: String,
}
pub enum ErrorKind {
    TooBig,
}
pub type Result<T> = std::result::Result<T, Error>;