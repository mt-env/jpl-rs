pub mod codegen;
pub mod driver;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod typechecker;

pub struct Spanned<T> {
    pub offset: usize,
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new_ref<'ast>(allocator: &impl Allocator<'ast>, offset: usize, value: T) -> &'ast Self {
        allocator.alloc(Self { offset, value })
    }

    pub const fn new(offset: usize, value: T) -> Self {
        Self { offset, value }
    }
}

pub trait Allocator<'ast> {
    fn alloc<T>(&self, value: T) -> &'ast T;
}
