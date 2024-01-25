use std::collections::HashMap;

use regex::Regex;

use crate::{
    filters::{FilterValue, Filters},
    operator::Operator,
    order_dir::OrderDir,
    Expansions,
};

pub struct QueryOptions<'a> {
    pub filters: Filters,
    pub expansions: Expansions<'a>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: Option<&'a str>,
    pub order_dir: Option<OrderDir>,
}

impl<'a> QueryOptions<'a> {
    pub fn new() -> Self {
        Self {
            filters: Filters(Box::default()),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        }
    }

    fn flatten_grouped_filters(
        grouped_filters: HashMap<Box<str>, Vec<(Operator, FilterValue)>>,
    ) -> HashMap<Box<str>, (Box<str>, Operator, FilterValue)> {
        let mut result = HashMap::new();

        for (key, values) in grouped_filters.into_iter() {
            let mut i = 0;

            for (operator, value) in values.into_iter() {
                let enumerated_key = if i == 0 {
                    key.clone()
                } else {
                    format!("{}__{}", &key, i).into_boxed_str()
                };

                result.insert(enumerated_key, (key.clone(), operator, value));

                i += 1;
            }
        }

        result
    }

    fn build_filters(filters: Filters) -> (Box<str>, HashMap<Box<str>, FilterValue>) {
        if filters.is_empty() {
            return ("".into(), HashMap::new());
        }

        let grouped_filters: HashMap<Box<str>, Vec<(Operator, FilterValue)>> = filters
            .0
            .into_vec()
            .into_iter()
            .filter_map(|(unsafe_key, (operator, value))| {
                let key = sanitize(&unsafe_key)?;

                Some((key.to_string().into_boxed_str(), (operator, value)))
            })
            .fold(HashMap::new(), |mut acc, (key, (operator, value))| {
                match value {
                    FilterValue::Escaped(_) | FilterValue::Unsafe(_) => match acc.get_mut(&key) {
                        Some(values) => values.push((operator, value)),
                        None => {
                            acc.insert(key, vec![(operator, value)]);
                        }
                    },
                    FilterValue::EscapedList(_) => {
                        match operator {
                            // Ignore any operator that's not an array operator when we have an array of values
                            Operator::ContainsAny | Operator::Inside => match acc.get_mut(&key) {
                                Some(values) => values.push((operator, value)),
                                None => {
                                    acc.insert(key, vec![(operator, value)]);
                                }
                            },
                            _ => {}
                        };
                    }
                };

                acc
            });

        let filters = QueryOptions::flatten_grouped_filters(grouped_filters);

        let mut filters_query_vec = filters
            .iter()
            .filter_map(|(enumerated_key, (key, operator, value))| {
                let variable_ident = to_variable_ident(enumerated_key);

                match value {
                    FilterValue::Escaped(_) => Some(format!(
                        "{} {} {}",
                        key,
                        operator,
                        format!("${}", variable_ident)
                    )),
                    FilterValue::Unsafe(value) => Some(format!("{} {} {}", key, operator, value)),
                    FilterValue::EscapedList(_) => {
                        // Ignore any operator that's not an array operator when we have an array of values
                        match operator {
                            Operator::ContainsAny | Operator::Inside => {
                                Some(format!("{} {} ${}", key, operator, variable_ident))
                            }
                            _ => return None,
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        filters_query_vec.sort_unstable();

        let filters_query = filters_query_vec.join(" AND ");

        let variables = filters
            .into_iter()
            .filter_map(|(enumerated_key, (_, _, value))| {
                let key = to_variable_ident(&enumerated_key);

                match value {
                    FilterValue::Escaped(_) | FilterValue::EscapedList(_) => Some((key, value)),
                    FilterValue::Unsafe(_) => None,
                }
            })
            .collect();

        (
            format!("WHERE {}", filters_query).into_boxed_str(),
            variables,
        )
    }

    pub fn build(
        self,
        table_name: &str,
        unsafe_columns: &[&str],
    ) -> (Box<str>, HashMap<Box<str>, FilterValue>) {
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

        let (filters_query, variables) = QueryOptions::build_filters(self.filters);

        if !filters_query.is_empty() {
            push_query_str(&mut query, &filters_query);
        }

        if let Some(Some(order_by)) = self.order_by.map(|ob| sanitize(ob)) {
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
    let regex = Regex::new(r"[\w\.]+").unwrap();

    let value = regex.captures(value)?.get(0)?.as_str();

    Some(value)
}

fn to_variable_ident(value: &str) -> Box<str> {
    value.replace('.', "_").into_boxed_str()
}
