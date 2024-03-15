use actix_web::{HttpResponse, Responder};

pub async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index() {
        let app =
            test::init_service(App::new().route("/handlername", web::get().to(index_handler)))
                .await;

        let req = test::TestRequest::get().uri("/handlername").to_request();
        let resp = app.call(req).await.unwrap();
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, world!");
    }
}
