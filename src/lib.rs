pub mod driver;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod typechecker;

pub struct Spanned<T> {
    pub offset: usize,
    pub value: T,
}
