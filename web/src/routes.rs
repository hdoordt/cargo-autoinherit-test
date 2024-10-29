use crate::controllers::tasks;
use crate::middlewares::auth::{auth, SecurityAddon};
use crate::state::AppState;
use axum::{middleware, Router};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
modifiers(&SecurityAddon),
    tags(
(name = tasks::OPENAPI_TAG, description = "Task API endpoints"),
    )
)]
struct ApiDoc;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/greet") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(tasks::create))
        .routes(routes!(tasks::create_batch))
        .routes(routes!(tasks::delete))
        .routes(routes!(tasks::update))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .routes(routes!(tasks::read_all))
        .routes(routes!(tasks::read_one))
        .split_for_parts();
    router
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
        .with_state(app_state)
}
