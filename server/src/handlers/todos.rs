pub mod todos {
    use crate::database::Database;
    use crate::handlers::response::response;
    use crate::models::todos::todos::Todo;
    use crate::models::todos::todos::TodoChangeSet;
    use crate::utils::pagination::PaginationParams;
    use actix_web::web::Data;
    use actix_web::web::Json;
    use actix_web::web::Path;
    use actix_web::web::Query;
    use actix_web::{delete, get, post, Error, HttpResponse};

    type InputTodo = TodoChangeSet;

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
                "Internal server error",
            )),
        }
    }

    #[post("")]
    async fn create(
        db: Data<Database>,
        Json(input_todo): Json<InputTodo>,
    ) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::create(&mut db, &input_todo);

        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Created(), 201, &todo)),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Todo not created",
            )),
        }
    }

    #[get("/{id}")]
    async fn get(db: Data<Database>, todo_id: Path<i32>) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::read(&mut db, todo_id.into_inner());

        match result {
            Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
            Err(_) => Ok(response::error(
                &mut HttpResponse::NotFound(),
                404,
                "Todo not found",
            )),
        }
    }

    #[post("/{id}")]
    async fn update(
        db: Data<Database>,
        Json(input_todo): Json<InputTodo>,
        todo_id: Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::update(&mut db, todo_id.into_inner(), &input_todo);

        match result {
            Ok(todo) => Ok(response::success(&mut HttpResponse::Ok(), 200, &todo)),
            Err(_) => Ok(response::error(
                &mut HttpResponse::InternalServerError(),
                500,
                "Todo not updated",
            )),
        }
    }

    #[delete("/{id}")]
    async fn delete(db: Data<Database>, todo_id: Path<i32>) -> Result<HttpResponse, Error> {
        let mut db = db.get_connection();
        let result = Todo::delete(&mut db, todo_id.into_inner());

        if result.is_ok() {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::InternalServerError().finish())
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
