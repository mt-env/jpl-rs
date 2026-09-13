use std::collections::HashMap;

use bumpalo::Bump;

use crate::typechecker::ast::TypeValue;

pub(super) enum NameInfo<'src, 'ast> {
    Value(&'ast TypeValue<'src, 'ast>),
    Struct(Vec<(&'src str, &'ast TypeValue<'src, 'ast>)>),
}

impl<'src, 'ast> NameInfo<'src, 'ast> {
    pub(super) fn new(ctx: &TypecheckCtx<'src, 'ast>, info: NameInfo<'src, 'ast>) -> &'ast Self {
        ctx.alloc(info)
    }
}

pub(super) struct TypecheckCtx<'src, 'ast> {
    alloc: &'ast Bump,
    env: HashMap<&'src str, &'ast NameInfo<'src, 'ast>>,
}

impl<'src, 'ast> TypecheckCtx<'src, 'ast> {
    pub(super) fn new(alloc: &'ast Bump) -> Self {
        Self {
            alloc,
            env: HashMap::new(),
        }
    }

    pub(super) fn alloc<A>(&self, value: A) -> &'ast A {
        self.alloc.alloc(value)
    }

    pub(super) fn add_struct_info(
        &mut self,
        name: &'src str,
        fields: Vec<(&'src str, &'ast TypeValue<'src, 'ast>)>,
    ) {
        self.env
            .insert(name, NameInfo::new(self, NameInfo::Struct(fields)));
    }

    pub(super) fn lookup(&self, name: &'src str) -> Option<&'ast NameInfo<'src, 'ast>> {
        self.env.get(name).copied()
    }
}
