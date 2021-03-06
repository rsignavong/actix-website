pub mod integration_one;
pub mod integration_two;
pub mod stream_response;
use actix_web::{http, web, App, HttpRequest, HttpResponse};

fn index(req: HttpRequest) -> HttpResponse {
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            return HttpResponse::Ok().into();
        }
    }
    HttpResponse::BadRequest().into()
}

fn main() {
    App::new().route("/", web::get().to(index));
}

// <unit-tests>
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[test]
    fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
            .to_http_request();

        let resp = test::block_on(index(req)).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = test::block_on(index(req)).unwrap();
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
// </unit-tests>
