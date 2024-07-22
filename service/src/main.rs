use axum::Router;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service("/", ServeDir::new("assets"));
    Ok(router.into())
}
