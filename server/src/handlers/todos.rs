pub mod todos {
    use crate::database::Database;
    use crate::handlers::response::response;
    use crate::models::todos::todos::{Todo, TodoChangeSet, TodoCreateSet};
    use crate::utils::pagination::PaginationParams;
    use actix_web::web::{Data, Json, Path, Query};
    use actix_web::{delete, get, post, Error, HttpResponse};
    use diesel::result::Error::NotFound;

    type CreateInputTodo = TodoCreateSet;
    type UpdateInputTodo = TodoChangeSet;

    #[get("")]
    async fn get_all(
        db: Data<Database>,
        Query(info): Query<PaginationParams>,
    ) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::read_all(&mut db, &info);

        let page = match info.page {
            Some(page) => page,
            None => 0,
        };
        let page_size = match info.page_size {
            Some(page_size) => page_size,
            None => PaginationParams::MAX_PAGE_SIZE as i64,
        };

        let total = match Todo::count(&mut db) {
            Ok(total) => total,
            Err(_) => 0,
        };

        match result {
            Ok(todos) => Ok(response::success_paginated(
                &mut HttpResponse::Ok(),
                200,
                &todos,
                page,
                page_size,
                total,
            )),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Something went wrong, could not retrieve todos",
            )),
        }
    }

    #[post("")]
    async fn create(
        db: Data<Database>,
        Json(input_todo): Json<CreateInputTodo>,
    ) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();

        let data = TodoCreateSet {
            title: input_todo.title.trim().to_string(),
            description: input_todo.description.trim().to_string(),
        };
        let result = Todo::create(&mut db, &data);

        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Created(), 201, &todo)),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Something went wrong, could not create todo",
            )),
        }
    }

    #[get("/{id}")]
    async fn get(db: Data<Database>, todo_id: Path<i32>) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::read(&mut db, todo_id.into_inner());

        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Ok(), 200, &todo)),
            Err(NotFound) => Ok(response::error(
                &mut HttpResponse::NotFound(),
                404,
                "Todo not found",
            )),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Something went wrong, could not get todo",
            )),
        }
    }

    #[post("/{id}")]
    async fn update(
        db: Data<Database>,
        Json(input_todo): Json<UpdateInputTodo>,
        todo_id: Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();

        let title = match input_todo.title {
            Some(title) => Some(title.trim().to_string()),
            None => None,
        };
        let description = match input_todo.description {
            Some(description) => Some(description.trim().to_string()),
            None => None,
        };
        let is_completed = match input_todo.is_completed {
            Some(is_completed) => Some(is_completed),
            None => None,
        };
        let data = TodoChangeSet {
            title,
            description,
            is_completed,
        };

        let result = Todo::update(&mut db, todo_id.into_inner(), &data);
        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Ok(), 200, &todo)),
            Err(NotFound) => Ok(response::error(
                &mut HttpResponse::NotFound(),
                404,
                "Todo not found",
            )),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Something went wrong, could not update todo",
            )),
        }
    }

    #[delete("/{id}")]
    async fn delete(db: Data<Database>, todo_id: Path<i32>) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::delete(&mut db, todo_id.into_inner());

        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Ok(), 200, &todo)),
            Err(NotFound) => Ok(response::error(
                &mut HttpResponse::NotFound(),
                404,
                "Todo not found",
            )),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Something went wrong, could not delete todo",
            )),
        }
    }

    pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
        return scope
            .service(get)
            .service(get_all)
            .service(create)
            .service(delete);
    }
}
