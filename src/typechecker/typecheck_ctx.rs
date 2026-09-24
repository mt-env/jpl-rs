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
        let f32_f32 = NameInfo::new(
            &current,
            NameInfo::Fn {
                params: vec![&TypeValue::Float],
                ret_ty: &TypeValue::Float,
            },
        );
        current.global_env.insert("sqrt", f32_f32);
        current.global_env.insert("exp", f32_f32);
        current.global_env.insert("sin", f32_f32);
        current.global_env.insert("cos", f32_f32);
        current.global_env.insert("tan", f32_f32);
        current.global_env.insert("asin", f32_f32);
        current.global_env.insert("acos", f32_f32);
        current.global_env.insert("atan", f32_f32);
        current.global_env.insert("log", f32_f32);

        // (f32, f32) -> f32: pow, atan2
        let f32_f32_f32 = NameInfo::new(
            &current,
            NameInfo::Fn {
                params: vec![&TypeValue::Float, &TypeValue::Float],
                ret_ty: &TypeValue::Float,
            },
        );
        current.global_env.insert("pow", f32_f32_f32);
        current.global_env.insert("atan2", f32_f32_f32);

        // i32 -> f32: to_float
        let i32_f32 = NameInfo::new(
            &current,
            NameInfo::Fn {
                params: vec![&TypeValue::Int],
                ret_ty: &TypeValue::Float,
            },
        );
        current.global_env.insert("to_float", i32_f32);

        // f32 -> i32: to_int
        let f32_i32 = NameInfo::new(
            &current,
            NameInfo::Fn {
                params: vec![&TypeValue::Float],
                ret_ty: &TypeValue::Int,
            },
        );
        current.global_env.insert("to_int", f32_i32);

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
    ) -> Result<(), TypeError<'src, 'ast>> {
        let fn_info = NameInfo::new(self, NameInfo::Fn { params, ret_ty });
        if let Some(_) = self.global_env.insert(name, &fn_info) {
            return Err(TypeError {
                offset: 0,
                value: TypeErrorKind::DuplicateIdentifier(name),
            });
        }
        Ok(())
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
