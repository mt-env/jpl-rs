use std::collections::HashMap;

use bumpalo::Bump;

use crate::typechecker::ast::{TypeError, TypeErrorKind, TypeValue};

pub(super) enum NameInfo<'src, 'ast> {
    Value(&'ast TypeValue<'src, 'ast>),
    Struct(Vec<(&'src str, &'ast TypeValue<'src, 'ast>)>),
    Fn {
        params: Vec<&'ast TypeValue<'src, 'ast>>,
        ret_ty: &'ast TypeValue<'src, 'ast>,
    },
}

impl<'src, 'ast> NameInfo<'src, 'ast> {
    pub(super) fn new(ctx: &TypecheckCtx<'src, 'ast>, info: NameInfo<'src, 'ast>) -> &'ast Self {
        ctx.alloc(info)
    }
}

pub(super) struct TypecheckCtx<'src, 'ast> {
    alloc: &'ast Bump,
    global_env: HashMap<&'src str, &'ast NameInfo<'src, 'ast>>,
    local_scopes: Vec<HashMap<&'src str, &'ast NameInfo<'src, 'ast>>>,
}

impl<'src, 'ast> TypecheckCtx<'src, 'ast> {
    pub(super) fn new(alloc: &'ast Bump) -> Self {
        let mut current = Self {
            alloc,
            global_env: HashMap::new(),
            local_scopes: Vec::new(),
        };

        current.add_struct_info(
            "rgba",
            vec![
                ("r", current.alloc.alloc(TypeValue::Float)),
                ("g", current.alloc.alloc(TypeValue::Float)),
                ("b", current.alloc.alloc(TypeValue::Float)),
                ("a", current.alloc.alloc(TypeValue::Float)),
            ],
        );

        current
            .global_env
            .insert("argnum", &NameInfo::Value(&TypeValue::Int));
        current.global_env.insert(
            "args",
            &NameInfo::Value(&TypeValue::Array {
                element_type: &TypeValue::Int,
                dimension: 1,
            }),
        );

        // f32 -> f32: sqrt, exp, sin, cos, tan, asin, acos, atan, and log
        current.add_fn_info("sqrt", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("exp", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("sin", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("cos", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("tan", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("asin", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("acos", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("atan", vec![&TypeValue::Float], &TypeValue::Float);
        current.add_fn_info("log", vec![&TypeValue::Float], &TypeValue::Float);

        // (f32, f32) -> f32: pow, atan2
        current.add_fn_info(
            "pow",
            vec![&TypeValue::Float, &TypeValue::Float],
            &TypeValue::Float,
        );
        current.add_fn_info(
            "atan2",
            vec![&TypeValue::Float, &TypeValue::Float],
            &TypeValue::Float,
        );

        // i32 -> f32: to_float
        current.add_fn_info("to_float", vec![&TypeValue::Int], &TypeValue::Float);
        // f32 -> i32: to_int
        current.add_fn_info("to_int", vec![&TypeValue::Float], &TypeValue::Int);

        current
    }

    pub(super) fn alloc<A>(&self, value: A) -> &'ast A {
        self.alloc.alloc(value)
    }

    pub(super) fn add_struct_info(
        &mut self,
        name: &'src str,
        fields: Vec<(&'src str, &'ast TypeValue<'src, 'ast>)>,
    ) {
        self.global_env
            .insert(name, NameInfo::new(self, NameInfo::Struct(fields)));
    }

    pub(super) fn add_fn_info(
        &mut self,
        name: &'src str,
        params: Vec<&'ast TypeValue<'src, 'ast>>,
        ret_ty: &'ast TypeValue<'src, 'ast>,
    ) {
        let fn_info = NameInfo::new(self, NameInfo::Fn { params, ret_ty });
        self.global_env.insert(name, &fn_info);
    }

    pub(super) fn lookup(&self, name: &'src str) -> Option<&'ast NameInfo<'src, 'ast>> {
        for scope in self.local_scopes.iter().rev() {
            if let Some(data) = scope.get(name) {
                return Some(data);
            }
        }

        self.global_env.get(name).copied()
    }

    pub(super) fn push_scope(&mut self) {
        self.local_scopes.push(HashMap::new());
    }

    pub(super) fn pop_scope(&mut self) {
        self.local_scopes.pop();
    }

    pub(super) fn bind(
        &mut self,
        offset: usize,
        name: &'src str,
        value: &'ast TypeValue<'src, 'ast>,
    ) -> Result<(), TypeError<'src, 'ast>> {
        let info = NameInfo::new(self, NameInfo::Value(value));
        let curr_scope = self.local_scopes.last_mut().unwrap_or(&mut self.global_env);
        if let Some(_) = curr_scope.insert(name, info) {
            return Err(TypeError {
                offset,
                value: TypeErrorKind::DuplicateIdentifier(name),
            });
        }
        Ok(())
    }
}
