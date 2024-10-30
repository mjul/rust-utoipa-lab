# rust-utoipa-lab
Exposing a HTTP REST API and its OpenAPI schema using [Utoipa](https://github.com/juhaku/utoipa), 
Axum and Rust.

Utoipa is a crate to help build OpenAPI documentation for an API and 
expose its schema and an application for exploring the API.

It is a fairly nice library with a couple of surprises (until you RTFM).

Note that you have to accept some redundancy in documenting the API and its routes
and setting up the routes for serving the API. 

Here are some examples to get started.

## Nesting with Derive
This example uses derive macros to nest the API documentation (schema)
and the ordinary Axum Router to set up the paths for serving the API over HTTP.

## Nesting with `OpenApiRouter`
This example uses the `OpenApiRouter` to nest the API documentation (schema)
and build the router at the same time.
Then, it extracts the router and documentation in time for serving.
