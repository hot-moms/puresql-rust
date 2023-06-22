# puresql-rust #
***Pure (Vec[u&] based) implementation of SQL Query Builder on Rust***

Install:
```puresql-rust = { git = "https://github.com/hot-moms/"}```


**puresql-rust** is a lightweight and efficient SQL query builder written in Rust.

It offers a pure, lightweight string-based implementation for constructing SQL statements and queries.
With puresql-rust, users can easily generate complex and robust SQL queries with a very unsafe, but simple and intuitive interface.


### Functions:

```
pub fn from(initial_query: &str) -> QueryBuilder
pub fn equal_to<T: ToSql + Sync>(&mut self, column_name: &str, object: Option<&'a T>)
pub fn field_is(&mut self, column_name: &str, is_null: bool)
pub fn compare_with(&mut self, column_name: &str, is_descending: bool, is_timestamp: bool)
pub fn any_of<T: ToSql + Sync + 'a>(&mut self, column_name: &str, objects: &'a Vec<T>)
pub fn contained_by<T: ToSql + Sync>
pub fn between(&mut self, column_name: &str, range: &'a RangeIncluse<i32>)
pub fn order_by(&mut self, sort_by: &str, ordering: &str, is_search: bool)
pub fn limit(&mut self, limit: u8)
pub fn offset(&mut self, offset: i32)
pub fn custom_query(&mut self, query_: &str)    <-- push your custom query
```


### Example ###

#### Code:
```
    let mut sql = QueryBuilder::from(QUERY);          <-- QUERY - some basic query like 'SELECT * FROM something'
    sql.equal_to("item", [same as item column]);
    sql.contained_by(
        "some_array",
        [array],
        [bool: full_match?],
    );

    sql.any_of("anyof_array", [array]);
    sql.between("year", [rangeInclusive: 1000..2000]);


    await database.execute(&sql.query, &sql.params);  <-- special getters for query and params
```

---

_2023, Archie Iwakura (hot-moms)_
