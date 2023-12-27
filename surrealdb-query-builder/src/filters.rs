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

pub struct Filters<'a, T>(pub HashMap<&'a str, (Operator, T)>);

impl<'a, T> Deref for Filters<'a, T> {
    type Target = HashMap<&'a str, (Operator, T)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> DerefMut for Filters<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: Clone> Into<Filters<'a, T>> for &'a [(&'a str, (Operator, T))] {
    fn into(self) -> Filters<'a, T> {
        Filters(self.iter().cloned().collect::<HashMap<_, _>>())
    }
}

impl<'a, T> Into<Filters<'a, T>> for Box<[(&'a str, (Operator, T))]> {
    fn into(self) -> Filters<'a, T> {
        Filters(self.into_vec().into_iter().collect())
    }
}

impl<'a, T> Into<Filters<'a, T>> for Vec<(&'a str, (Operator, T))> {
    fn into(self) -> Filters<'a, T> {
        Filters(self.into_iter().collect())
    }
}

impl<'a, T: Clone> Into<Filters<'a, T>> for &'a [(&'a str, T)] {
    fn into(self) -> Filters<'a, T> {
        Filters(
            self.into_iter()
                .map(|(key, value)| (*key, (Operator::Eq, value.clone())))
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl<'a, T> Into<Filters<'a, T>> for Box<[(&'a str, T)]> {
    fn into(self) -> Filters<'a, T> {
        Filters(
            self.into_vec()
                .into_iter()
                .map(|(key, value)| (key, (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}

impl<'a, T> Into<Filters<'a, T>> for Vec<(&'a str, T)> {
    fn into(self) -> Filters<'a, T> {
        Filters(
            self.into_iter()
                .map(|(key, value)| (key, (Operator::Eq, value)))
                .collect::<HashMap<_, _>>(),
        )
    }
}
