use actix_web::{App, web, HttpResponse, HttpServer, Responder, http::header::ContentType};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(get_index)))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(get_index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::html())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
