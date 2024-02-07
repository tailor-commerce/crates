use std::{
    borrow::Cow,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::Serialize;

use crate::operator::Operator;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FilterValueKind<'a> {
    String(Cow<'a, str>),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
}

impl<'a> Into<FilterValueKind<'a>> for &'a str {
    fn into(self) -> FilterValueKind<'a> {
        FilterValueKind::String(self.into())
    }
}

impl<'a> Into<FilterValueKind<'a>> for String {
    fn into(self) -> FilterValueKind<'a> {
        FilterValueKind::String(self.into())
    }
}

impl Into<FilterValueKind<'_>> for i64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Int(self)
    }
}

impl Into<FilterValueKind<'_>> for u64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::UInt(self)
    }
}

impl Into<FilterValueKind<'_>> for f64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Float(self)
    }
}

impl Into<FilterValueKind<'_>> for i32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Int(self.into())
    }
}

impl Into<FilterValueKind<'_>> for u32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::UInt(self.into())
    }
}

impl Into<FilterValueKind<'_>> for f32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Float(self.into())
    }
}

impl Into<FilterValueKind<'_>> for bool {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Bool(self)
    }
}

impl Into<FilterValueKind<'_>> for &i64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Int(*self)
    }
}

impl Into<FilterValueKind<'_>> for &u64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::UInt(*self)
    }
}

impl Into<FilterValueKind<'_>> for &f64 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Float(*self)
    }
}

impl Into<FilterValueKind<'_>> for &i32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Int((*self).into())
    }
}

impl Into<FilterValueKind<'_>> for &u32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::UInt((*self).into())
    }
}

impl Into<FilterValueKind<'_>> for &f32 {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Float((*self).into())
    }
}

impl Into<FilterValueKind<'_>> for &bool {
    fn into(self) -> FilterValueKind<'static> {
        FilterValueKind::Bool(*self)
    }
}

impl Display for FilterValueKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValueKind::String(value) => value.fmt(f),
            FilterValueKind::Int(value) => value.fmt(f),
            FilterValueKind::UInt(value) => value.fmt(f),
            FilterValueKind::Float(value) => value.fmt(f),
            FilterValueKind::Bool(value) => value.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FilterValue<'a> {
    Escaped(FilterValueKind<'a>),
    Unsafe(FilterValueKind<'a>),
    EscapedList(Cow<'a, [FilterValueKind<'a>]>),
}

impl<'a> Into<FilterValue<'a>> for FilterValueKind<'a> {
    fn into(self) -> FilterValue<'a> {
        FilterValue::Escaped(self)
    }
}

impl<'a> Into<FilterValue<'a>> for &'a str {
    fn into(self) -> FilterValue<'a> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for String {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for i64 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for u64 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for f64 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for i32 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for u32 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for f32 {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue<'static>> for bool {
    fn into(self) -> FilterValue<'static> {
        FilterValue::Escaped(self.into())
    }
}

impl<'a, T> Into<FilterValue<'a>> for &'a [T]
where
    &'a T: Into<FilterValueKind<'a>>,
{
    fn into(self) -> FilterValue<'a> {
        FilterValue::EscapedList(self.into_iter().map(|s| s.into()).collect())
    }
}

impl<'a, T: Into<FilterValueKind<'a>>> Into<FilterValue<'a>> for Box<[T]> {
    fn into(self) -> FilterValue<'a> {
        FilterValue::EscapedList(self.into_vec().into_iter().map(|s| s.into()).collect())
    }
}

impl<'a, T: Into<FilterValueKind<'a>>> Into<FilterValue<'a>> for Vec<T> {
    fn into(self) -> FilterValue<'a> {
        self.into_boxed_slice().into()
    }
}

impl Display for FilterValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValue::Escaped(value) => value.fmt(f),
            FilterValue::Unsafe(value) => value.fmt(f),
            FilterValue::EscapedList(values) => format!(
                "[{}]",
                values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            )
            .fmt(f),
        }
    }
}

#[derive(Default)]
pub struct Filters<'fv, 'key>(pub Box<[(Cow<'key, str>, (Operator, FilterValue<'fv>))]>);

impl<'fv, 'key> Deref for Filters<'fv, 'key> {
    type Target = Box<[(Cow<'key, str>, (Operator, FilterValue<'fv>))]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'fv, 'key> DerefMut for Filters<'fv, 'key> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'fv, 'key, T: Into<FilterValue<'fv>>, S: Into<Cow<'key, str>>> Into<Filters<'fv, 'key>>
    for Vec<(S, (Operator, T))>
{
    fn into(self) -> Filters<'fv, 'key> {
        Filters(
            self.into_iter()
                .map(|(key, (operator, value))| (key.into(), (operator, value.into())))
                .collect(),
        )
    }
}

impl<'fv, 'key, T: Into<FilterValue<'fv>>, S: Into<Cow<'key, str>>> Into<Filters<'fv, 'key>>
    for Box<[(S, (Operator, T))]>
{
    fn into(self) -> Filters<'fv, 'key> {
        self.into_vec().into()
    }
}

impl<'fv, 'key, T: Clone + Into<FilterValue<'fv>>, S: Into<Cow<'key, str>> + Clone>
    Into<Filters<'fv, 'key>> for &[(S, (Operator, T))]
{
    fn into(self) -> Filters<'fv, 'key> {
        let b: Box<[_]> = self.into();
        b.into()
    }
}

impl<'fv, 'key, T: Into<FilterValue<'fv>>, S: Into<Cow<'key, str>>> Into<Filters<'fv, 'key>>
    for Vec<(S, T)>
{
    fn into(self) -> Filters<'fv, 'key> {
        Filters(
            self.into_iter()
                .map(|(key, value)| (key.into().into(), (Operator::Eq, value.into())))
                .collect(),
        )
    }
}

impl<'fv, 'key, T: Into<FilterValue<'fv>>, S: Into<Cow<'key, str>>> Into<Filters<'fv, 'key>>
    for Box<[(S, T)]>
{
    fn into(self) -> Filters<'fv, 'key> {
        self.into_vec().into()
    }
}
