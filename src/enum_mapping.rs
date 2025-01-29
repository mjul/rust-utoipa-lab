use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

/// Application state structure.
#[derive(Clone)]
pub struct AppState {}

impl AppState {
    fn new() -> Self {
        Self {}
    }
}

/// Index route handler.
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
    use axum::{extract::State, routing::get, Json, Router};
    use serde::Serialize;
    use utoipa::{OpenApi, ToSchema};

    /// API documentation using OpenAPI.
    #[derive(utoipa::OpenApi)]
    #[openapi(
        info(
            title = "Rust Enums to JavaScript Simple Types API",
            description = "API demonstrating enums as JS simple types."
        ),
        paths(get_enums)
    )]
    struct ApiDoc;

    /// API router function.
    pub(crate) fn router() -> Router<AppState> {
        Router::new().route("/enums", get(get_enums))
    }

    /// Return the OpenAPI schema
    pub(crate) fn schema() -> utoipa::openapi::OpenApi {
        ApiDoc::openapi()
    }

    /// Example response structure for enums.
    #[derive(Serialize, ToSchema)]
    pub struct EnumResponse {
        untagged: Vec<UntaggedEnum>,
        tagged: Vec<TaggedEnum>,
        discriminator: Vec<DiscriminatorEnum>,
        discriminator_add_type_field: Vec<DiscriminatorAddTypeFieldEnum>,
    }

    #[derive(Serialize, ToSchema)]
    pub enum TaggedEnum {
        Int(i64),
        Bool(bool),
        Str(String),
    }

    /// Here the type of the variant is not specified in the output (it is "untagged").
    /// This is useful for enums that are used as simple types in JavaScript where
    /// the type of the value specifies the missing tag implicitly (if it is needed).
    #[derive(Serialize, ToSchema)]
    #[serde(untagged)]
    pub enum UntaggedEnum {
        Int(i64),
        Bool(bool),
        Str(String),
    }

    /// This serializes the enum as maps with a `type` field as the tag and a `value` field for the value, *e.g.*
    ///
    /// ```json
    ///   "discriminator": [
    ///     {
    ///       "type": "number",
    ///       "value": 123
    ///     },
    ///     {
    ///       "type": "Boolean",
    ///       "value": false
    ///     },
    ///     {
    ///       "type": "String",
    ///       "value": "foo"
    ///     }
    ///   ]
    /// ```
    #[derive(Serialize, ToSchema)]
    #[serde(tag = "type", content = "value")]
    pub enum DiscriminatorEnum {
        // You can use standard serde notation
        #[serde(rename = "number")]
        Int(i64),
        #[serde(rename = "Boolean")]
        Bool(bool),
        #[serde(rename = "String")]
        Str(String),
    }

    /// This serializes the enum as a map with an additional `type` field as the tag added
    /// to the map representing its value, *e.g.*
    ///
    /// ```json
    ///   {
    ///     "_tag": "Foo",
    ///     "value": 1
    ///   },
    ///   {
    ///     "_tag": "Bar",
    ///     "value": 2
    ///   }
    /// ```
    #[derive(Serialize, ToSchema)]
    #[serde(tag = "_tag")]
    pub enum DiscriminatorAddTypeFieldEnum {
        Foo(FooStruct),
        Bar(BarStruct),
    }

    #[derive(Serialize, ToSchema)]
    pub struct FooStruct {
        value: i64,
    }

    #[derive(Serialize, ToSchema)]
    pub struct BarStruct {
        value: i64,
    }

    /// Handler function to get enum values.
    #[utoipa::path(
        get,
        path = "/api/enums",
        responses(
            (status = 200, description = "Enum values retrieved", body = EnumResponse)
        )
    )]
    pub async fn get_enums(_: State<AppState>) -> Json<EnumResponse> {
        Json(EnumResponse {
            untagged: vec![
                UntaggedEnum::Int(123),
                UntaggedEnum::Bool(false),
                UntaggedEnum::Str(String::from("foo")),
            ],
            tagged: vec![
                TaggedEnum::Int(123),
                TaggedEnum::Bool(false),
                TaggedEnum::Str(String::from("foo")),
            ],
            discriminator: vec![
                DiscriminatorEnum::Int(123),
                DiscriminatorEnum::Bool(false),
                DiscriminatorEnum::Str(String::from("foo")),
            ],
            discriminator_add_type_field: vec![
                DiscriminatorAddTypeFieldEnum::Foo(FooStruct { value: 1 }),
                DiscriminatorAddTypeFieldEnum::Bar(BarStruct { value: 2 }),
            ],
        })
    }
}
