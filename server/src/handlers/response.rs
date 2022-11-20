pub mod response {
    use actix_web::{HttpResponse, HttpResponseBuilder};
    use serde::Serialize;
    use serde_json::json;

    pub fn error(res: &mut HttpResponseBuilder, code: u16, message: &str) -> HttpResponse {
        res.json(json!({ "error": code, "message": message }))
    }

    pub fn success<T: Serialize>(
        res: &mut HttpResponseBuilder,
        code: u16,
        result: &T,
    ) -> HttpResponse {
        res.json(json!({ "success": code, "result": &result }))
    }

    pub fn success_paginated<T: Serialize>(
        res: &mut HttpResponseBuilder,
        code: u16,
        result: &T,
        page: i64,
        page_size: i64,
        total: i64,
    ) -> HttpResponse {
        res.json(json!({
            "success": code,
            "result": &result,
            "page": page,
            "page_size": page_size,
            "total": total
        }))
    }
}
