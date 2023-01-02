use actix_web::{App, web, HttpResponse, HttpServer, Responder, http::header::ContentType};
use serde::{Deserialize, Serialize};

mod gcd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
            App::new()
                .route("/", web::get().to(get_index))
                .route("/gcd", web::post().to(post_gcd))
        )
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[derive(Debug, Deserialize, Serialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type(ContentType::html())
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The gratest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd::gcd(form.n, form.m)
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(response)
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
    use actix_web::test;

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
