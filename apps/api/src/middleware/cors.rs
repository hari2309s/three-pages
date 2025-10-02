use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    let origins: Vec<_> = allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    if origins.is_empty() {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
