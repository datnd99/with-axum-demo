use axum::{routing::get, Router};

use crate::server::state::AppState;

pub fn create_router_app(state: AppState) -> Router {
//   let router = Router::new()
//     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    let router: Router<AppState> = Router::new().route("/",get(root));
    router.with_state(state)
}

async fn root() -> &'static str {
    "Hello, World!"
}
