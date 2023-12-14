use std::{collections::HashMap, fmt::Display};

pub enum OrderDir {
    Asc,
    Desc,
}

#[derive(Clone)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
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
        }
    }
}

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

pub struct QueryOptions<'a, T: Into<FilterValue<'a>>> {
    filters: HashMap<&'a str, (Operator, T)>,
    expansions: &'a [(&'a str, &'a str)],
    limit: Option<usize>,
    offset: Option<usize>,
    order_by: Option<&'a str>,
    order_dir: Option<OrderDir>,
}

impl<'a, T: Into<FilterValue<'a>> + Clone> QueryOptions<'a, T> {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
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
        columns: &[&str],
    ) -> (Box<str>, HashMap<Box<str>, &'a str>) {
        let expansions = self
            .expansions
            .into_iter()
            .map(|(expansion, key)| format!("({}) AS {}", key, expansion).into_boxed_str())
            .collect::<Vec<_>>()
            .join(",");

        let expansions = if expansions.is_empty() {
            expansions
        } else {
            format!(",{}", expansions)
        };

        let mut query = format!(
            "SELECT {}{} FROM {}",
            columns.join(","),
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
                .map(
                    |(key, (operator, value))| match <T as Into<FilterValue<'a>>>::into(value) {
                        FilterValue::Escaped(_) => {
                            format!("{} {} {}", key, operator, format!("${}", key))
                        }
                        FilterValue::Unsafe(value) => {
                            format!("{} {} {}", key, operator, value)
                        }
                    },
                )
                .collect::<Vec<_>>();

            filters_query_vec.sort_unstable();

            let filters_query = filters_query_vec.join(" AND ");

            variables = self
                .filters
                .into_iter()
                .filter_map(
                    |(key, (_, value))| match <T as Into<FilterValue<'a>>>::into(value) {
                        FilterValue::Escaped(value) => {
                            Some((format!("${}", key).into_boxed_str(), value))
                        }
                        FilterValue::Unsafe(_) => None,
                    },
                )
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

#[cfg(test)]
mod tests {
    use surrealdb::{
        engine::local::{Db, Mem},
        opt::Config,
        Surreal,
    };

    use super::*;

    async fn set_up_db() -> Surreal<Db> {
        let db = Surreal::new::<Mem>(Config::default().strict())
            .await
            .unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db.query("DEFINE NAMESPACE test")
            .query("DEFINE DATABASE test")
            .query("DEFINE TABLE users SCHEMAFULL")
            .query("DEFINE FIELD name ON TABLE users TYPE string")
            .query("DEFINE FIELD age ON TABLE users TYPE int")
            .query("DEFINE FIELD year_of_birth ON TABLE users TYPE int")
            .query("DEFINE FIELD month_of_birth ON TABLE users TYPE int")
            .query("DEFINE FIELD day_of_birth ON TABLE users TYPE int")
            .await
            .unwrap();

        db
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_one_filter() {
        let opts = QueryOptions {
            filters: HashMap::from([("name", (Operator::Eq, "tester testermann"))]),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users WHERE name = $name ORDER BY id ASC LIMIT 10 START 0"
        );
        assert_eq!(query.1, [("$name".into(), "tester testermann")].into());

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_accepts_unsafe_filters() {
        let opts = QueryOptions {
            filters: HashMap::from([(
                "name",
                (Operator::Eq, FilterValue::Unsafe("\"unsafe person\"")),
            )]),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users WHERE name = \"unsafe person\" ORDER BY id ASC LIMIT 10 START 0"
        );
        assert_eq!(query.1, [].into());

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_multiple_filters() {
        let opts = QueryOptions {
            filters: HashMap::from([
                ("name", (Operator::Eq, "tester testermann")),
                ("id", (Operator::Ne, "1")),
            ]),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users WHERE id != $id AND name = $name ORDER BY id ASC LIMIT 10 START 0"
        );

        assert_eq!(
            query.1,
            [("$name".into(), "tester testermann"), ("$id".into(), "1")].into()
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_filters() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_limit() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: None,
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id ASC START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_offset() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: None,
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id ASC LIMIT 10"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_order_by() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: None,
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_order_dir() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: None,
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_order_dir_desc() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Desc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id DESC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_order_dir_asc() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_filters_with_the_correct_operators() {
        let opts = QueryOptions {
            filters: HashMap::from([
                ("name", (Operator::Eq, "tester testermann")),
                ("id", (Operator::Ne, "1")),
                ("age", (Operator::Gt, "1")),
                ("year_of_birth", (Operator::Ge, "5")),
                ("month_of_birth", (Operator::Lt, "10")),
                ("day_of_birth", (Operator::Le, "10")),
            ]),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM users WHERE age > $age AND day_of_birth <= $day_of_birth AND id != $id AND month_of_birth < $month_of_birth AND name = $name AND year_of_birth >= $year_of_birth ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_creates_the_correct_variables() {
        let opts = QueryOptions {
            filters: HashMap::from([
                ("name", (Operator::Eq, "tester testermann")),
                ("id", (Operator::Ne, "1")),
                ("age", (Operator::Gt, "1")),
                ("year_of_birth", (Operator::Ge, "5")),
                ("month_of_birth", (Operator::Lt, "10")),
                ("day_of_birth", (Operator::Le, "10")),
            ]),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.1,
            [
                ("$name".into(), "tester testermann"),
                ("$id".into(), "1"),
                ("$age".into(), "1"),
                ("$year_of_birth".into(), "5"),
                ("$month_of_birth".into(), "10"),
                ("$day_of_birth".into(), "10")
            ]
            .into()
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_supports_expansions() {
        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[("purchases", "->purchased.out")],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name,(->purchased.out) AS purchases FROM users ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_works_with_multiple_expansions() {
        let orders_query = QueryOptions {
            filters: HashMap::from([("user", (Operator::Eq, FilterValue::Unsafe("$parent.id")))]),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        }
        .build("orders", &["*"]);

        let opts = QueryOptions::<&str> {
            filters: HashMap::new(),
            expansions: &[
                ("purchases", "->purchased.out"),
                ("orders", orders_query.0.as_ref()),
            ],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("users", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name,(->purchased.out) AS purchases,(SELECT * FROM orders WHERE user = $parent.id) AS orders FROM users ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref())
            .bind(query.1)
            .bind(orders_query.1)
            .await
            .unwrap();
    }
}
