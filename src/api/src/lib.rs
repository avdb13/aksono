use std::sync::Arc;

use aksono_common::app;
use router::RouterExt as _;

pub mod c2s;
pub mod id;
pub mod push;
pub mod s2s;
pub mod wk;

pub mod error;
pub mod router;

type Result<T, E = error::Error> = core::result::Result<T, E>;

pub fn build_routes(app: app::App) -> axum::Router {
    let router = router::Router::new()
        .route(wk::get_discovery_information)
        .route(wk::get_support_information);

    router.into_inner().with_state(Arc::new(app))
}
