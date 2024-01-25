use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    ContainsAny,
    Inside,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Eq => write!(f, "="),
            Operator::Ne => write!(f, "!="),
            Operator::Gt => write!(f, ">"),
            Operator::Ge => write!(f, ">="),
            Operator::Lt => write!(f, "<"),
            Operator::Le => write!(f, "<="),
            Operator::ContainsAny => write!(f, "CONTAINSANY"),
            Operator::Inside => write!(f, "INSIDE"),
        }
    }
}

struct OperatorVisitor;

impl<'de> serde::de::Visitor<'de> for OperatorVisitor {
    type Value = Operator;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Operator")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "=" => Ok(Operator::Eq),
            "!=" => Ok(Operator::Ne),
            ">" => Ok(Operator::Gt),
            ">=" => Ok(Operator::Ge),
            "<" => Ok(Operator::Lt),
            "<=" => Ok(Operator::Le),
            _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Operator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(OperatorVisitor)
    }
}
