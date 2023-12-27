use serde::Deserialize;

#[derive(Clone)]
pub enum OrderDir {
    Asc,
    Desc,
}

struct OrderDirVisitor;

impl<'de> serde::de::Visitor<'de> for OrderDirVisitor {
    type Value = OrderDir;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("OrderDir")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "asc" => Ok(OrderDir::Asc),
            "desc" => Ok(OrderDir::Desc),
            _ => Err(E::invalid_value(serde::de::Unexpected::Str(v), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for OrderDir {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(OrderDirVisitor)
    }
}
