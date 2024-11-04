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

## Native JavaScript types and Rust Enums
JavaScript has typed values, so you can put *e.g.* a string, number or Boolean into
a field all the same. In TypeScript, you can declare this like `value: number | boolean | string`
Rust uses an `enum` for this. 

By default, the Utoipa serializer to JSON (Serde) will use "tagged" enums, encoding a value
as a key-value pair, where the key is the enum variant. The key is called the tag.

You can also use "untagged" representation (with a Serde attribute) to serialize as the
raw values without the keys. This works when the values are natural JavaScript types
and the type of the value corresponds to the missing tag.

See [enum_of_js_simple_types.rs](src/enum_of_js_simple_types.rs)


