pub mod stock;
pub mod stock_model;

#[cfg(test)]
mod tests {
    use crate::common::app_state::AppState;
    use crate::feature::stock::stock::get_detail_stock;
    use axum::body::Body;
    use axum::handler::Handler;
    use axum::http::{Method, Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;
    use crate::router::init_routes;

    #[tokio::test]
    async fn test_get_stock() {
        let app_state = AppState::init().await;

        let app: Router<()> = init_routes(app_state);


        let req = Request::builder()
            .method(Method::GET)
            .uri("/stock/get-list-stock")
            .header("content-type", "application/json")
            .body(Body::empty())
            // .body(Body::from(
            //     r#"{
            //         "username": "johndoe87"
            //     }"#,
            // ))
            .unwrap();

        let response = app.oneshot(req).await;
        println!("{:?}", response);
        let status = response.map_or_else(|_| StatusCode::NOT_FOUND, |v|v.status());
        debug_assert_eq!(status, 200);
    }
}
