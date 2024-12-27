use std::sync::Arc;

use aksono_common::app;
use router::RouterExt as _;

mod wk;

pub mod error;
pub mod router;

pub(crate) type Result<T, E = error::Error> = core::result::Result<T, E>;

pub fn build_routes(app: app::App) -> axum::Router {
    let router = router::Router::new().route(wk::get_discovery_information);

    router.into_inner().with_state(Arc::new(app))
}
