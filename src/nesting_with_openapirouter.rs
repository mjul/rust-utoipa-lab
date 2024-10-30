//! Utoipa can nest OpenAPI routers by declaring the nested API docs on in the derive macro on the
//! parent API doc or by calling `nest` on the router instance.
//!
//! Mixing these two approaches easily leads to duplicates in the OpenAPI schema, or
//! have the Axum API routing and the documentation routing not match.
//!
//! I recommend to stick to one strategy.
//!
//! In this example we use the Utoipa OpenApiRouter to nest the API documentation
//! and set up routes at the same time

use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa_axum::router::OpenApiRouter;
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
    // Here, we build the top-level OpenAPI router under which to mount the API
    let (api_router, api_doc) = OpenApiRouter::default()
        .nest("/api", api::openapi_router())
        .split_for_parts();

    // The routes in that are already relative to the root, so we merge it
    // rather than nest it (in fact, we could use it directly without the new/merge combination).
    Router::new()
        .merge(api_router)
        .route_service("/", get(index))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState::new())
}

mod api {
    use super::AppState;
    use utoipa::OpenApi;
    use utoipa_axum::router::OpenApiRouter;

    // Note that we do not add the nesting here in the derive macro,
    // it is set in the openapi_router() below.
    //
    //         nest(
    //             (path="/api/widgets", api=widgets::WidgetsApi)
    //         )
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Nesting with the Utoipa OpenAPI router",
        description = "This API is built by nesting API docs via the Utoipa OpenAPI Router"
    ))]
    pub struct ApiDoc;

    /// Return the OpenAPI Router with the routes and the OpenAPI schema.
    pub(crate) fn openapi_router() -> OpenApiRouter<AppState> {
        OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/widgets", widgets::openapi_router())
    }

    mod widgets {
        use super::AppState;
        use axum::extract::State;
        use axum::Json;
        use utoipa::{OpenApi, ToSchema};
        use utoipa_axum::router::OpenApiRouter;
        use utoipa_axum::routes;

        #[derive(OpenApi)]
        #[openapi(
            info(title = "Widgets", description = "Provide access to Widget instances."),
            paths(get_widgets)
        )]
        pub struct WidgetsApi;

        pub(super) fn openapi_router() -> OpenApiRouter<AppState> {
            OpenApiRouter::new().routes(routes!(get_widgets))
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
            Json(Widgets {
                names: vec![String::from("Utoipa"), String::from("Axum"), name],
            })
        }
    }
}
