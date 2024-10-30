//! Utoipa can nest OpenAPI routers by declaring the nested API docs on in the derive macro on the
//! parent API doc or by calling `nest` on the router instance.
//!
//! Mixing these two approaches easily leads to duplicates in the OpenAPI schema, or
//! have the Axum API routing and the documentation routing not match.
//!
//! I recommend to stick to one strategy.
//!
//! In this example we use the derive macros, and the Axum Router for setting the routes.

use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    // Just something to show the use of AppState
    name: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            name: String::from("Foo"),
        }
    }
}

async fn index() -> &'static str {
    "Hello, World"
}

/// Get the application router with an API route and Swagger (OpenAPI) schema and UI.
///
// It is recommended to use generic Router<S> when working with nested routers
// in Axum to have the exact type inferred by the compiler.
// Try replacing this with `router() -> Router<AppState>` to see why.
pub(crate) fn router<S>() -> Router<S> {
    let api_router = api::router();
    let api_doc = api::schema();

    Router::new()
        .route_service("/", get(index))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http())
        .with_state(AppState::new())
}

mod api {
    use crate::nesting_with_derive::AppState;
    use axum::Router;
    use utoipa::OpenApi;

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Nesting with derive macros",
            description = "This API is built by nesting API docs via the macros"
        ),
        nest(
            (path="/api/widgets", api=widgets::WidgetsApi)
        ))]
    pub struct ApiDoc;

    /// Return the Axum Router
    pub(crate) fn router() -> Router<AppState> {
        Router::new().nest("/widgets", widgets::router())
    }

    /// Return the OpenAPI schema
    pub(crate) fn schema() -> utoipa::openapi::OpenApi {
        ApiDoc::openapi()
    }

    mod widgets {
        use crate::nesting_with_derive::AppState;
        use axum::extract::State;
        use axum::routing::get;
        use axum::{Json, Router};
        use utoipa::{OpenApi, ToSchema};

        #[derive(OpenApi)]
        #[openapi(
            info(title = "Widgets", description = "Provide access to Widget instances."),
            paths(get_widgets)
        )]
        pub struct WidgetsApi;

        pub(super) fn router() -> Router<AppState> {
            Router::new().route("/", get(get_widgets))
        }

        #[derive(ToSchema, serde::Serialize)]
        pub struct Widgets {
            names: Vec<String>,
        }

        // For some reason this endpoints mounts at /api/widgets with no trailing "/"
        // both when the path is set to "" and "/".
        // This is documented in the Axum Router nest function (stripping it is
        // useful for serving files).
        // There is a OriginalUri extractor available to get the real URI in the handler
        // but it is better to just model it according to the framework with no
        // trailing / to avoid the confusion.
        // This took me a while to figure out.
        #[utoipa::path(
            get,
            path = "",
            responses(
                (status = 200, description = "Widgets found successfully", body = Widgets),
            ),
            params()
        )]
        async fn get_widgets(State(state): State<AppState>) -> Json<Widgets> {
            let name: String = state.name.clone();
            Json(Widgets { names: vec![name] })
        }
    }
}
