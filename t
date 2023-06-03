[1mdiff --git a/src/routes/mod.rs b/src/routes/mod.rs[m
[1mindex d8be887..28e0275 100644[m
[1m--- a/src/routes/mod.rs[m
[1m+++ b/src/routes/mod.rs[m
[36m@@ -6,7 +6,8 @@[m [mmod mirror_custom_header;[m
 mod mirror_user_agent;[m
 mod path_variables;[m
 mod query_params;[m
[31m-[m
[32m+[m[32mmod read_middleware_custom_header;[m
[32m+[m[32mmod set_middleware_custom_header;[m
 use axum::{[m
     body::Body,[m
     http::Method,[m
[36m@@ -21,6 +22,9 @@[m [muse mirror_custom_header::mirror_custom_header;[m
 use mirror_user_agent::mirror_user_agent;[m
 use path_variables::{hard_coded_path, path_variables};[m
 use query_params::query_params;[m
[32m+[m[32muse read_middleware_custom_header::read_middleware_custom_header;[m
[32m+[m[32muse set_middleware_custom_header::set_middleware_custom_header;[m
[32m+[m
 use tower_http::cors::{Any, CorsLayer};[m
 [m
 #[derive(Clone)][m
[36m@@ -49,5 +53,9 @@[m [mpub fn create_routes() -> Router<(), Body> {[m
         .route("/mirror_custom_header", get(mirror_custom_header))[m
         .route("/middleware_message", get(middleware_message))[m
         .layer(cors) // this layer will affect all routes above (and not those below)[m
[31m-        .layer(Extension(shared_data)) // use a layer and wrap data to share on axum::Extension - all above will have access to this data[m
[32m+[m[32m        .layer(Extension(shared_data)) // use a layer to wrap data and share on other routes using axum::Extension - all above will have access to this data[m
[32m+[m[32m        .route([m
[32m+[m[32m            "/read_middleware_custom_header",[m
[32m+[m[32m            get(read_middleware_custom_header),[m
[32m+[m[32m        )[m
 }[m
