pub mod filters;
pub mod operator;
pub mod order_dir;
pub mod query_options;

pub type Expansions<'a> = &'a [(&'a str, &'a str)];

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_decimal::Decimal;
    use serde::Deserialize;
    use surrealdb::{
        engine::local::{Db, Mem},
        opt::Config,
        Surreal,
    };

    use crate::{
        filters::{FilterValue, Filters},
        operator::Operator,
        order_dir::OrderDir,
        query_options::QueryOptions,
    };

    async fn set_up_db() -> Surreal<Db> {
        let db = Surreal::new::<Mem>(Config::default().strict())
            .await
            .unwrap();

        db.use_ns("test").use_db("test").await.unwrap();

        db.query("DEFINE NAMESPACE test")
            .query("DEFINE DATABASE test")
            .query("DEFINE TABLE user SCHEMAFULL")
            .query("DEFINE FIELD name ON TABLE user TYPE string")
            .await
            .unwrap();

        db
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_one_filter() {
        let opts = QueryOptions {
            filters: Filters(Box::from([(
                "name".into(),
                (
                    Operator::Eq,
                    FilterValue::Escaped("tester testermann".into()),
                ),
            )])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE name = $name ORDER BY id ASC LIMIT 10 START 0"
        );
        assert_eq!(
            query.1,
            [("name".into(), "tester testermann".into())].into()
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_accepts_unsafe_filters() {
        let opts = QueryOptions {
            filters: Filters(Box::from([(
                "name".into(),
                (
                    Operator::Eq,
                    FilterValue::Unsafe("\"unsafe person\"".into()),
                ),
            )])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE name = \"unsafe person\" ORDER BY id ASC LIMIT 10 START 0"
        );
        assert_eq!(query.1, [].into());

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_multiple_filters() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("name".into(), (Operator::Eq, "tester testermann".into())),
                ("id".into(), (Operator::Ne, "1".into())),
            ])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE id != $id AND name = $name ORDER BY id ASC LIMIT 10 START 0"
        );

        assert_eq!(
            query.1,
            [
                ("name".into(), "tester testermann".into()),
                ("id".into(), "1".into())
            ]
            .into()
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_filters() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_limit() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: None,
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id ASC START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_offset() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: None,
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id ASC LIMIT 10"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_order_by() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: None,
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_no_order_dir() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: None,
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_order_dir_desc() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Desc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id DESC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_builds_the_correct_query_with_order_dir_asc() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).await.unwrap();
    }

    #[tokio::test]
    async fn it_filters_with_the_correct_operators() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("name".into(), (Operator::Eq, "tester testermann".into())),
                ("id".into(), (Operator::Ne, "1".into())),
                ("age".into(), (Operator::Gt, "1".into())),
                ("year_of_birth".into(), (Operator::Ge, "5".into())),
                ("month_of_birth".into(), (Operator::Lt, "10".into())),
                ("day_of_birth".into(), (Operator::Le, "10".into())),
            ])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE age > $age AND day_of_birth <= $day_of_birth AND id != $id AND month_of_birth < $month_of_birth AND name = $name AND year_of_birth >= $year_of_birth ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_creates_the_correct_variables() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("name".into(), (Operator::Eq, "tester testermann".into())),
                ("id".into(), (Operator::Ne, "1".into())),
                ("age".into(), (Operator::Gt, "1".into())),
                ("year_of_birth".into(), (Operator::Ge, "5".into())),
                ("month_of_birth".into(), (Operator::Lt, 10.into())),
                ("day_of_birth".into(), (Operator::Le, 10.into())),
                ("is_active".into(), (Operator::Eq, true.into())),
            ])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.1,
            HashMap::from([
                ("name".into(), "tester testermann".into()),
                ("id".into(), "1".into()),
                ("age".into(), "1".into()),
                ("year_of_birth".into(), "5".into()),
                ("month_of_birth".into(), 10.into()),
                ("day_of_birth".into(), 10.into()),
                ("is_active".into(), true.into())
            ])
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_supports_expansions() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[("purchases", "->purchased.out")],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name,(->purchased.out) AS purchases FROM user ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_works_with_multiple_expansions() {
        let orders_query = QueryOptions {
            filters: Filters(Box::from([(
                "user".into(),
                (Operator::Eq, FilterValue::Unsafe("$parent.id".into())),
            )])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        }
        .build("orders", &["*"]);

        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[
                ("purchases", "->purchased.out"),
                ("orders", orders_query.0.as_ref()),
            ],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name,(->purchased.out) AS purchases,(SELECT * FROM orders WHERE user = $parent.id) AS orders FROM user ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref())
            .bind(query.1)
            .bind(orders_query.1)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn it_sanitizes_filter_keys() {
        let opts = QueryOptions {
            filters: Filters(Box::from([(
                "name = \"hello\"; DELETE user:hello; SELECT * FROM user WHERE name = \"hello\""
                    .into(),
                (Operator::Eq, "whatever".into()),
            )])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE name = $name ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;
        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_allows_dots_in_the_filter_key() {
        let opts = QueryOptions {
            filters: Filters(Box::from([(
                "tag.name".into(),
                (Operator::Eq, "whatever".into()),
            )])),
            expansions: &[],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "tag"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,tag FROM user WHERE tag.name = $tag_name ORDER BY id ASC LIMIT 10 START 0"
        );

        assert_eq!(query.1.get("tag_name"), Some(&"whatever".into()));

        let db = set_up_db().await;
        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_sanitizes_expansion_keys() {
        let opts = QueryOptions {
            filters: Filters(Box::new([])),
            expansions: &[(
                "purchased_items = \"hello\"; DELETE user:hello; SELECT * FROM user WHERE name = \"hello\"",
                "->purchased.out",
            )],
            limit: Some(10),
            offset: Some(0),
            order_by: Some("id"),
            order_dir: Some(OrderDir::Asc),
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name,(->purchased.out) AS purchased_items FROM user ORDER BY id ASC LIMIT 10 START 0"
        );

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_can_filter_values_in_arrays() {
        let opts = QueryOptions {
            filters: Filters(Box::from([(
                "tags".into(),
                (Operator::ContainsAny, vec!["tag1", "tag2"].into()),
            )])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE tags CONTAINSANY $tags"
        );

        assert_eq!(query.1.get("tags").unwrap(), &vec!["tag1", "tag2"].into());
    }

    #[tokio::test]
    async fn it_ignores_array_filters_for_non_array_operators() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                (
                    "not_equal".into(),
                    (Operator::Ne, vec!["value1", "value2"].into()),
                ),
                (
                    "less_than_or_equal".into(),
                    (Operator::Le, vec!["value1", "value2"].into()),
                ),
                (
                    "less_than".into(),
                    (Operator::Lt, vec!["value1", "value2"].into()),
                ),
                (
                    "greater_than_or_equal".into(),
                    (Operator::Ge, vec!["value1", "value2"].into()),
                ),
                (
                    "greater_than".into(),
                    (Operator::Gt, vec!["value1", "value2"].into()),
                ),
                (
                    "equal".into(),
                    (Operator::Eq, vec!["value1", "value2"].into()),
                ),
                (
                    "contains_any".into(),
                    (Operator::ContainsAny, vec!["value1", "value2"].into()),
                ),
                (
                    "inside_operator".into(),
                    (Operator::Inside, vec!["value1", "value2"].into()),
                ),
            ])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE contains_any CONTAINSANY $contains_any AND inside_operator INSIDE $inside_operator"
        );

        assert_eq!(
            query.1,
            [
                ("inside_operator".into(), vec!["value1", "value2"].into()),
                ("contains_any".into(), vec!["value1", "value2"].into()),
            ]
            .into()
        );
    }

    #[tokio::test]
    async fn it_allows_multiple_filters_for_the_same_field() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("price".into(), (Operator::Le, 20.into())),
                ("price".into(), (Operator::Ge, 10.into())),
                ("price".into(), (Operator::Inside, vec![5, 6].into())),
            ])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        };

        let query = opts.build("user", &["id", "name"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT id,name FROM user WHERE price <= $price AND price >= $price__1 AND price INSIDE $price__2"
        );

        assert_eq!(query.1.get("price").unwrap(), &20.into());
        assert_eq!(query.1.get("price__1").unwrap(), &10.into());
        assert_eq!(query.1.get("price__2").unwrap(), &vec![5, 6].into());

        let db = set_up_db().await;

        db.query(query.0.as_ref()).bind(query.1).await.unwrap();
    }

    #[tokio::test]
    async fn it_serializes_the_variables_correctly() {
        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("age".into(), (Operator::Gt, 20.into())),
                ("age".into(), (Operator::Lt, 40.into())),
                (
                    "tags".into(),
                    (Operator::ContainsAny, vec!["tag1", "tag2"].into()),
                ),
                ("profession".into(), (Operator::Eq, "tester".into())),
            ])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        };

        let query = opts.build("test", &["*"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT * FROM test WHERE age < $age__1 AND age > $age AND profession = $profession AND tags CONTAINSANY $tags"
        );
        assert_eq!(
            query.1,
            HashMap::from([
                ("age".into(), 20.into()),
                ("age__1".into(), 40.into()),
                ("profession".into(), "tester".into()),
                ("tags".into(), vec!["tag1", "tag2"].into())
            ])
        );

        let db = set_up_db().await;

        db.query(
            r#"
            DEFINE TABLE test;

            CREATE test CONTENT {
                name: "tester testermann",
                age: 21,
                tags: ["tag1", "tag3"],
                profession: "tester",
            };

            CREATE test CONTENT {
                name: "checkster assertson",
                age: 19,
                tags: ["tag1", "tag2"],
                profession: "tester",
            };

            CREATE test CONTENT {
                name: "penelopen performance",
                age: 30,
                tags: ["tag2"],
                profession: "pen tester",
            };
        "#,
        )
        .await
        .unwrap();

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestValue {
            name: String,
            age: i32,
            tags: Vec<String>,
            profession: String,
        }

        let mut result = db.query(query.0.as_ref()).bind(query.1).await.unwrap();
        let result: Vec<TestValue> = result.take(0).unwrap();

        assert_eq!(
            result,
            vec![TestValue {
                name: "tester testermann".into(),
                age: 21,
                tags: vec!["tag1".into(), "tag3".into()],
                profession: "tester".into(),
            }]
        )
    }

    #[tokio::test]
    async fn it_works_with_decimals() {
        let db = set_up_db().await;

        db.query(
            r"
            DEFINE TABLE decimal_test SCHEMAFULL;
            DEFINE FIELD price ON decimal_test TYPE decimal;

            CREATE decimal_test:1 SET price = 15dec;
        ",
        )
        .await
        .unwrap();

        let opts = QueryOptions {
            filters: Filters(Box::from([
                ("price".into(), (Operator::Le, Decimal::from(20).into())),
                ("price".into(), (Operator::Ge, Decimal::from(10).into())),
            ])),
            expansions: &[],
            limit: None,
            offset: None,
            order_by: None,
            order_dir: None,
        };

        let query = opts.build("decimal_test", &["price"]);

        assert_eq!(
            query.0.as_ref(),
            "SELECT price FROM decimal_test WHERE price <= $price AND price >= $price__1"
        );

        assert_eq!(query.1.get("price").unwrap(), &Decimal::from(20).into());
        assert_eq!(query.1.get("price__1").unwrap(), &Decimal::from(10).into());

        let mut response = db.query(query.0.as_ref()).bind(query.1).await.unwrap();

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestValue {
            pub price: Decimal,
        }

        let result: Vec<TestValue> = response.take(0).unwrap();

        assert_eq!(
            result.first(),
            Some(&TestValue {
                price: Decimal::from(15)
            })
        );
    }
}
