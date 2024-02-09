use crate::{from_value::FromValue, Value};

use std::{any::TypeId, fmt};

pub trait Visitor<T>: 'static {
    fn visit(&self, value: &Value);
}

impl<F, R, T> Visitor<(T,)> for F
where
    F: Fn(T) -> R + 'static,
    T: FromValue,
    R: fmt::Debug + 'static,
{
    fn visit(&self, value: &Value) {
        let Some(arg1) = FromValue::from_value(value) else {
            return;
        };
        let result = self(arg1);

        // If R is not the unit type
        if TypeId::of::<R>() != TypeId::of::<()>() {
            // ... print result
            println!("{result:?}");
        }
    }
}

impl<F, R, T1, T2> Visitor<(T1, T2)> for F
where
    F: Fn(T1, T2) -> R + 'static,
    T1: FromValue,
    T2: FromValue,
    R: fmt::Debug + 'static,
{
    fn visit(&self, value: &Value) {
        let Some(arg1) = FromValue::from_value(value) else {
            return;
        };
        let Some(arg2) = FromValue::from_value(value) else {
            return;
        };
        let result = self(arg1, arg2);

        // If R is not the unit type
        if TypeId::of::<R>() != TypeId::of::<()>() {
            // ... print result
            println!("{result:?}");
        }
    }
}
