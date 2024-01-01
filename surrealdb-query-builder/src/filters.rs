use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::operator::Operator;

#[derive(Clone)]
pub enum FilterValue {
    Escaped(Box<str>),
    Unsafe(Box<str>),
}

impl Into<FilterValue> for Box<str> {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self)
    }
}

impl Into<FilterValue> for String {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for &str {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Display for FilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValue::Escaped(value) => value.fmt(f),
            FilterValue::Unsafe(value) => value.fmt(f),
        }
    }
}

impl FilterValue {
    pub fn as_str(&self) -> &str {
        match self {
            FilterValue::Escaped(value) => value.as_ref(),
            FilterValue::Unsafe(value) => value.as_ref(),
        }
    }
}

#[derive(Default)]
pub struct Filters<T>(pub HashMap<Box<str>, (Operator, T)>);

impl<T> Deref for Filters<T> {
    type Target = HashMap<Box<str>, (Operator, T)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Filters<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S: Into<Box<str>>> Into<Filters<T>> for Box<[(S, (Operator, T))]> {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_vec()
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        )
    }
}

impl<T, S: Into<Box<str>>> Into<Filters<T>> for Vec<(S, (Operator, T))> {
    fn into(self) -> Filters<T> {
        self.into_boxed_slice().into()
    }
}

impl<T: Clone, S: Into<String> + Clone> Into<Filters<T>> for &[(S, (Operator, T))] {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_iter()
                .cloned()
                .map(|(key, value)| (key.into().into_boxed_str(), value))
                .collect(),
        )
    }
}

impl<T, S: Into<String>> Into<Filters<T>> for Box<[(S, T)]> {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_vec()
                .into_iter()
                .map(|(key, value)| (key.into().into_boxed_str(), (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl<T, S: Into<String>> Into<Filters<T>> for Vec<(S, T)> {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_iter()
                .map(|(key, value)| (key.into().into_boxed_str(), (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}
