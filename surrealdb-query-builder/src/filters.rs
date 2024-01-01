use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::operator::Operator;

#[derive(Clone)]
pub enum FilterValue<'a> {
    Escaped(&'a str),
    Unsafe(&'a str),
}

impl<'a> Into<FilterValue<'a>> for &'a str {
    fn into(self) -> FilterValue<'a> {
        FilterValue::Escaped(self)
    }
}

impl Display for FilterValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValue::Escaped(value) => value.fmt(f),
            FilterValue::Unsafe(value) => value.fmt(f),
        }
    }
}

impl<'a> FilterValue<'a> {
    pub fn as_str(&self) -> &'a str {
        match self {
            FilterValue::Escaped(value) => value,
            FilterValue::Unsafe(value) => value,
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

impl<T: Clone> Into<Filters<T>> for &[(Box<str>, (Operator, T))] {
    fn into(self) -> Filters<T> {
        Filters(self.iter().cloned().collect::<HashMap<_, _>>())
    }
}

impl<T> Into<Filters<T>> for Box<[(Box<str>, (Operator, T))]> {
    fn into(self) -> Filters<T> {
        Filters(self.into_vec().into_iter().collect())
    }
}

impl<T> Into<Filters<T>> for Vec<(Box<str>, (Operator, T))> {
    fn into(self) -> Filters<T> {
        Filters(self.into_iter().collect())
    }
}

impl<T> Into<Filters<T>> for Box<[(Box<str>, T)]> {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_vec()
                .into_iter()
                .map(|(key, value)| (key, (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl<T> Into<Filters<T>> for Vec<(Box<str>, T)> {
    fn into(self) -> Filters<T> {
        Filters(
            self.into_iter()
                .map(|(key, value)| (key, (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}
