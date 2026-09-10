use std::collections::HashMap;

use bumpalo::Bump;

use crate::typechecker::ast::TypeValue;

pub(super) struct TypecheckCtx<'src, 'ast> {
    alloc: &'ast Bump,
    env: HashMap<&'src str, &'ast TypeValue<'src, 'ast>>, // TODO this might need to be changed for
                                                          // structs/functions
}

impl<'src, 'ast> TypecheckCtx<'src, 'ast> {
    pub(super) fn alloc<A>(&self, value: A) -> &'ast A {
        self.alloc.alloc(value)
    }
}
