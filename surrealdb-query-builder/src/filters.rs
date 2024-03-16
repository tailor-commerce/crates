use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use rust_decimal::Decimal;
use serde::Serialize;

use crate::operator::Operator;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FilterValueKind {
    String(Box<str>),
    Int(i64),
    UInt(u64),
    Float(f64),
    #[serde(serialize_with = "serialize_decimal")]
    Decimal(Decimal),
    Bool(bool),
}

fn serialize_decimal<S>(d: &Decimal, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    surrealdb::sql::Number::Decimal(*d).serialize(s)
}

impl Into<FilterValueKind> for &str {
    fn into(self) -> FilterValueKind {
        FilterValueKind::String(self.into())
    }
}

impl Into<FilterValueKind> for String {
    fn into(self) -> FilterValueKind {
        FilterValueKind::String(self.into())
    }
}

impl Into<FilterValueKind> for Box<str> {
    fn into(self) -> FilterValueKind {
        FilterValueKind::String(self)
    }
}

impl Into<FilterValueKind> for i64 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Int(self)
    }
}

impl Into<FilterValueKind> for u64 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::UInt(self)
    }
}

impl Into<FilterValueKind> for f64 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Float(self)
    }
}

impl Into<FilterValueKind> for i32 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Int(self.into())
    }
}

impl Into<FilterValueKind> for u32 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::UInt(self.into())
    }
}

impl Into<FilterValueKind> for f32 {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Float(self.into())
    }
}

impl Into<FilterValueKind> for Decimal {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Decimal(self)
    }
}

impl Into<FilterValueKind> for bool {
    fn into(self) -> FilterValueKind {
        FilterValueKind::Bool(self)
    }
}

impl Display for FilterValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterValueKind::String(value) => value.fmt(f),
            FilterValueKind::Int(value) => value.fmt(f),
            FilterValueKind::UInt(value) => value.fmt(f),
            FilterValueKind::Float(value) => value.fmt(f),
            FilterValueKind::Decimal(value) => value.fmt(f),
            FilterValueKind::Bool(value) => value.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FilterValue {
    Escaped(FilterValueKind),
    Unsafe(FilterValueKind),
    EscapedList(Box<[FilterValueKind]>),
}

impl Into<FilterValue> for FilterValueKind {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self)
    }
}

impl Into<FilterValue> for &str {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for String {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for Box<str> {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for i64 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for u64 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for f64 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for i32 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for u32 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for f32 {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for Decimal {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl Into<FilterValue> for bool {
    fn into(self) -> FilterValue {
        FilterValue::Escaped(self.into())
    }
}

impl<T: Into<FilterValueKind>> Into<FilterValue> for Box<[T]> {
    fn into(self) -> FilterValue {
        FilterValue::EscapedList(self.into_vec().into_iter().map(|s| s.into()).collect())
    }
}

impl<T: Into<FilterValueKind>> Into<FilterValue> for Vec<T> {
    fn into(self) -> FilterValue {
        self.into_boxed_slice().into()
    }
}

impl Display for FilterValue {
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
pub struct Filters(pub Box<[(Box<str>, (Operator, FilterValue))]>);

impl Deref for Filters {
    type Target = Box<[(Box<str>, (Operator, FilterValue))]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Filters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Into<FilterValue>, S: Into<Box<str>>> Into<Filters> for Vec<(S, (Operator, T))> {
    fn into(self) -> Filters {
        Filters(
            self.into_iter()
                .map(|(key, (operator, value))| (key.into(), (operator, value.into())))
                .collect(),
        )
    }
}

impl<T: Into<FilterValue>, S: Into<Box<str>>> Into<Filters> for Box<[(S, (Operator, T))]> {
    fn into(self) -> Filters {
        self.into_vec().into()
    }
}

impl<T: Clone + Into<FilterValue>, S: Into<Box<str>> + Clone> Into<Filters>
    for &[(S, (Operator, T))]
{
    fn into(self) -> Filters {
        let b: Box<[_]> = self.into();
        b.into()
    }
}

impl<T: Into<FilterValue>, S: Into<String>> Into<Filters> for Vec<(S, T)> {
    fn into(self) -> Filters {
        Filters(
            self.into_iter()
                .map(|(key, value)| (key.into().into_boxed_str(), (Operator::Eq, value.into())))
                .collect(),
        )
    }
}

impl<T: Into<FilterValue>, S: Into<String>> Into<Filters> for Box<[(S, T)]> {
    fn into(self) -> Filters {
        self.into_vec().into()
    }
}
