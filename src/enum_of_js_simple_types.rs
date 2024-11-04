//! Rust enums to JavaScript simple types.

use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    fn new() -> Self {
        Self {}
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
    use super::AppState;
    use axum::extract::State;
    use axum::routing::get;
    use axum::{Json, Router};
    use utoipa::{OpenApi, ToSchema};

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Rust Enums to JavaScript simple types",
            description = "This API uses Rust enums",
        ),
        paths(get_enums_untagged, get_enums_tagged)
    )]
    pub struct ApiDoc;

    /// Return the Axum Router
    pub(crate) fn router() -> Router<AppState> {
        Router::new()
            .route("/enums-tagged", get(get_enums_tagged))
            .route("/enums-untagged", get(get_enums_untagged))
    }

    /// Return the OpenAPI schema
    pub(crate) fn schema() -> utoipa::openapi::OpenApi {
        ApiDoc::openapi()
    }

    /// Return a list of values as JSON primitive types, default "tagged" serialization
    /// where a value is a make with the tag (as key) and the value.
    ///
    /// Example response:
    ///
    /// ```json
    /// {
    ///   "values": [
    ///     {
    ///       "Int": 123
    ///     },
    ///     {
    ///       "Boolean": true
    ///     },
    ///     {
    ///       "String": "Hello"
    ///     }
    ///   ]
    /// }
    /// ```
    #[derive(ToSchema, serde::Serialize)]
    pub struct EnumsTaggedResponse {
        values: Vec<TaggedValue>,
    }
    #[derive(serde::Serialize, ToSchema)]
    pub enum TaggedValue {
        Int(i64),
        Boolean(bool),
        String(String),
    }

    /// Return a list of values as JSON primitive types,
    /// serde "untagged" serialization without the keys but values only.
    ///
    /// Example response:
    ///
    /// ```json
    /// {
    ///   "values": [
    ///     123,
    ///     true,
    ///     "Hello"
    ///   ]
    /// }
    /// ```
    #[derive(ToSchema, serde::Serialize)]
    pub struct EnumsUntaggedResponse {
        values: Vec<UntaggedValue>,
    }

    #[derive(serde::Serialize, ToSchema)]
    #[serde(untagged)]
    pub enum UntaggedValue {
        Int(i64),
        Boolean(bool),
        String(String),
    }

    #[utoipa::path(
        get,
        path = "/api/enums-tagged",
        responses(
                (status = 200, description = "Enums found", body = EnumsTaggedResponse),
        ),
        params()
    )]
    async fn get_enums_tagged(_: State<AppState>) -> Json<EnumsTaggedResponse> {
        Json(EnumsTaggedResponse {
            values: vec![
                TaggedValue::Int(123),
                TaggedValue::Boolean(true),
                TaggedValue::String("Hello".to_string()),
            ],
        })
    }

    #[utoipa::path(
        get,
        path = "/api/enums-untagged",
        responses(
                (status = 200, description = "Enums found", body = EnumsUntaggedResponse),
        ),
        params()
    )]
    async fn get_enums_untagged(_: State<AppState>) -> Json<EnumsUntaggedResponse> {
        Json(EnumsUntaggedResponse {
            values: vec![
                UntaggedValue::Int(123),
                UntaggedValue::Boolean(true),
                UntaggedValue::String("Hello".to_string()),
            ],
        })
    }
}
