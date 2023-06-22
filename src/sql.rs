// use std::str::FromStr;

// use tokio_postgres::types::ToSql;

use std::fmt::Write;

use tokio_postgres::types::ToSql;

pub struct QueryBuilder<'a> {
    pub query: String,
    pub params: Vec<&'a (dyn ToSql + Sync)>,
    pub args_count: usize,
}

impl<'a> QueryBuilder<'a> {
    pub fn from(initial_query: &str) -> QueryBuilder {
        QueryBuilder {
            query: String::from(initial_query),
            params: Vec::new(),
            args_count: 0,
        }
    }

    pub fn equal_to<T: ToSql + Sync>(&mut self, column_name: &str, object: Option<&'a T>) {
        if let Some(object) = object {
            self.params.push(object);
            self.custom_where(|x, sql| {
                sql.write_fmt(format_args!("{column_name} = ${x}")).unwrap();
            });
        }
    }
    /// Unsafe function, must be not first, because it doesn't modify params array
    pub fn field_is(&mut self, column_name: &str, is_null: bool) {
        self.custom_where(|_, sql| {
            sql.write_fmt(format_args!(
                "{column_name} IS {}",
                if is_null { "NULL" } else { "NOT NULL" }
            ))
            .unwrap();
        });
    }

    pub fn compare_with(&mut self, column_name: &str, is_descending: bool, is_timestamp: bool) {
        self.custom_where(|x, sql| {
            if is_timestamp {
                sql.write_fmt(format_args!(
                    "{column_name} {} TO_TIMESTAMP(${x}::int8::float8)",
                    if is_descending { "<" } else { ">" },
                ))
                .unwrap();
            } else {
                sql.write_fmt(format_args!(
                    "{column_name} {} ${x}",
                    if is_descending { "<" } else { ">" },
                ))
                .unwrap();
            }
        });
    }

    pub fn any_of<T: ToSql + Sync + 'a>(&mut self, column_name: &str, objects: &'a Vec<T>) {
        if !objects.is_empty() {
            self.params.push(objects);
            self.custom_where(|x, sql| {
                sql.write_fmt(format_args!("{column_name} = ANY(${x})",))
                    .unwrap();
            });
        }
    }

    pub fn contained_by<T: ToSql + Sync>(
        &mut self,
        column_name: &str,
        objects: &'a Vec<T>,
        full_match: bool,
    ) {
        if !objects.is_empty() {
            self.params.push(objects);
            self.custom_where(|x, sql| {
                sql.write_fmt(format_args!(
                    "{column_name} {} ${x}",
                    if full_match { "@>" } else { "&&" }
                ))
                .unwrap();
            });
        }
    }

    pub fn between(&mut self, column_name: &str, range: &'a std::ops::RangeInclusive<i32>) {
        self.params.push(range.start());
        self.params.push(range.end());
        self.custom_where(|x, sql| {
            sql.write_fmt(format_args!("{column_name} BETWEEN ${} AND ${x}", x - 1,))
                .unwrap();
        });
    }

    pub fn order_by(&mut self, sort_by: &str, ordering: &str) {
        self.query
            .write_fmt(format_args!(" ORDER BY {sort_by} {ordering}"))
            .unwrap();
    }

    pub fn limit(&mut self, limit: u8) {
        self.query
            .write_fmt(format_args!(" LIMIT {limit}"))
            .unwrap();
    }

    pub fn offset(&mut self, offset: i32) {
        self.query
            .write_fmt(format_args!(" OFFSET {offset}"))
            .unwrap();
    }

    fn custom_where(&mut self, query: impl Fn(usize, &mut String)) {
        self.args_count += 1;
        if self.args_count > 1 {
            self.query.push_str(" AND (");
            query(self.params.len(), &mut self.query);
            self.query.push(')');
        } else {
            self.query.push_str(" WHERE (");
            query(self.params.len(), &mut self.query);
            self.query.push(')');
        }
    }

    pub fn custom_query(&mut self, query_: &str) {
        self.query.push_str(query_);
    }
}
