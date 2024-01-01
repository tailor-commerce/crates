use std::collections::HashMap;

use regex::Regex;

use crate::{
    filters::{FilterValue, Filters},
    order_dir::OrderDir,
    Expansions,
};

pub struct QueryOptions<'a, T: Into<FilterValue>> {
    pub filters: Filters<T>,
    pub expansions: Expansions<'a>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: Option<&'a str>,
    pub order_dir: Option<OrderDir>,
}

impl<'a, T: Into<FilterValue> + Clone> QueryOptions<'a, T> {
    pub fn new() -> Self {
        Self {
            filters: Filters(HashMap::new()),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        }
    }

    pub fn build(
        self,
        table_name: &str,
        unsafe_columns: &[&str],
    ) -> (Box<str>, HashMap<Box<str>, Box<str>>) {
        let expansions = self
            .expansions
            .into_iter()
            .filter_map(|(unsafe_key, expansion)| {
                let key = sanitize(unsafe_key)?;

                Some(format!("({}) AS {}", expansion, key).into_boxed_str())
            })
            .collect::<Vec<_>>()
            .join(",");

        let expansions = if expansions.is_empty() {
            expansions
        } else {
            format!(",{}", expansions)
        };

        let mut query = format!(
            "SELECT {}{} FROM {}",
            unsafe_columns.join(","),
            expansions,
            table_name
        );

        let mut variables = HashMap::new();

        if self.filters.len() > 0 {
            push_query_str(&mut query, "WHERE");

            let mut filters_query_vec = self
                .filters
                .clone()
                .into_iter()
                .filter_map(|(unsafe_key, (operator, value))| {
                    let key = sanitize(&unsafe_key)?;

                    match <T as Into<FilterValue>>::into(value) {
                        FilterValue::Escaped(_) => {
                            Some(format!("{} {} {}", key, operator, format!("${}", key)))
                        }
                        FilterValue::Unsafe(value) => {
                            Some(format!("{} {} {}", key, operator, value))
                        }
                    }
                })
                .collect::<Vec<_>>();

            filters_query_vec.sort_unstable();

            let filters_query = filters_query_vec.join(" AND ");

            variables = self
                .filters
                .0
                .into_iter()
                .filter_map(|(unsafe_key, (_, value))| {
                    let key = sanitize(&unsafe_key)?;

                    match <T as Into<FilterValue>>::into(value) {
                        FilterValue::Escaped(value) => {
                            Some((key.to_string().into_boxed_str(), value))
                        }
                        FilterValue::Unsafe(_) => None,
                    }
                })
                .collect();

            push_query_str(&mut query, filters_query.as_ref());
        }

        if let Some(order_by) = self.order_by {
            push_query_str(&mut query, &format!("ORDER BY {}", order_by));

            if let Some(order_dir) = self.order_dir {
                match order_dir {
                    OrderDir::Asc => push_query_str(&mut query, "ASC"),
                    OrderDir::Desc => push_query_str(&mut query, "DESC"),
                }
            }
        }

        if let Some(limit) = self.limit {
            push_query_str(&mut query, format!("LIMIT {}", limit).as_str());
        }

        if let Some(offset) = self.offset {
            push_query_str(&mut query, format!("START {}", offset).as_str());
        }

        (query.into_boxed_str(), variables)
    }
}

fn push_query_str(query: &mut String, value: &str) {
    query.push(' ');
    query.push_str(value);
}

fn sanitize(value: &str) -> Option<&str> {
    let regex = Regex::new(r"\w+").unwrap();

    let value = regex.captures(value)?.get(0)?.as_str();

    Some(value)
}
