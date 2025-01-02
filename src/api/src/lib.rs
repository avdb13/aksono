use std::sync::Arc;

use aksono_common::app;
use router::RouterExt as _;

mod c2s;
mod s2s;
mod wk;

pub mod error;
pub mod router;

pub(crate) type Result<T, E = error::Error> = core::result::Result<T, E>;

pub fn build_routes(app: Arc<app::App>) -> axum::Router {
    let router = router::Router::new()
        .route(wk::get_discovery_information)
        .route(wk::get_support_information)
        .route(c2s::get_supported_versions);

    let router = router
        .route(c2s::register)
        .route(c2s::login)
        .route(c2s::get_login_token)
        .route(c2s::refresh)
        .route(c2s::logout)
        .route(c2s::logout_all);

    let router = router.route(s2s::get_server_version);

    router
        .into_inner()
        // .layer(middleware::from_fn(router::ui_auth_middleware))
        .with_state(app)
}
