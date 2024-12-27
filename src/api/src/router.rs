use ruma_handler::RumaHandler;

mod error;
mod incoming;
mod outgoing;
mod ruma_handler;

pub(crate) struct Router<S = ()>(axum::Router<S>);

pub(crate) trait RouterExt<S> {
    fn route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T, S>,
        T: 'static;
}

impl<S> RouterExt<S> for Router<S> {
    fn route<H, T>(self, handler: H) -> Self
    where
        H: RumaHandler<T, S>,
        T: 'static,
    {
        Router(handler.add_to_router(self.0))
    }
}

#[allow(clippy::new_without_default)]
impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) fn new() -> Self {
        Self(axum::Router::new())
    }

    pub(crate) fn into_inner(self) -> axum::Router<S> {
        self.0
    }
}
