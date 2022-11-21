pub mod todos {
    use crate::diesel::ExpressionMethods;
    use crate::schema::*;
    use crate::{database::Connection, utils::pagination::PaginationParams};
    use diesel::{insert_into, QueryDsl, QueryResult, RunQueryDsl};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone, Queryable, Identifiable)]
    #[diesel(table_name = todos, primary_key(id))]
    pub struct Todo {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub is_completed: bool,
    }

    impl Todo {
        pub fn count(db: &mut Connection) -> QueryResult<i64> {
            todos::table.count().get_result(db)
        }

        pub fn create(db: &mut Connection, item: &TodoCreateSet) -> QueryResult<Self> {
            use crate::schema::todos::dsl::*;

            insert_into(todos).values(item).get_result(db)
        }

        pub fn read(db: &mut Connection, item_id: i32) -> QueryResult<Self> {
            use crate::schema::todos::dsl::*;

            todos.filter(id.eq(item_id)).first(db)
        }

        pub fn read_all(
            db: &mut Connection,
            pagination_params: &PaginationParams,
        ) -> QueryResult<Vec<Self>> {
            use crate::schema::todos::dsl::*;

            let limit = match pagination_params.page_size {
                Some(page_size) => page_size,
                None => PaginationParams::MAX_PAGE_SIZE as i64,
            };

            let offset = match pagination_params.page {
                Some(page) => page * limit,
                None => 0,
            };

            todos.order_by(id).limit(limit).offset(offset).load(db)
        }

        pub fn update(
            db: &mut Connection,
            item_id: i32,
            item: &TodoChangeSet,
        ) -> QueryResult<Self> {
            use crate::schema::todos::dsl::*;

            diesel::update(todos.filter(id.eq(item_id)))
                .set(item)
                .get_result(db)
        }

        pub fn delete(db: &mut Connection, item_id: i32) -> QueryResult<usize> {
            use crate::schema::todos::dsl::*;

            diesel::delete(todos.filter(id.eq(item_id))).execute(db)
        }
    }

    #[derive(Debug, Deserialize, Insertable)]
    #[diesel(table_name = todos)]
    pub struct TodoCreateSet {
        pub title: String,
        pub description: String,
    }

    #[derive(AsChangeset, Debug, Deserialize, Insertable)]
    #[diesel(table_name = todos)]
    pub struct TodoChangeSet {
        pub title: Option<String>,
        pub description: Option<String>,
        pub is_completed: Option<bool>,
    }
}
