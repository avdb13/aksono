use std::{fmt::Debug, future::Future};

use super::{incoming::Incoming, outgoing::Outgoing};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::Method,
    response::IntoResponse,
    routing::{on, MethodFilter},
};
use ruma::api::{IncomingRequest, OutgoingResponse};

pub trait RumaHandler<T, S> {
    fn add_to_router(self, router: axum::Router<S>) -> axum::Router<S>;
}

macro_rules! impl_ruma_handler {
    ( $($ty:ident),* $(,)? ) => {
        #[async_trait]
        #[allow(non_snake_case)]
        impl<Q, $($ty,)* P, E, F, H, S>
            RumaHandler<($($ty,)* Incoming<Q>,), S> for H
        where
            Q: IncomingRequest + Send + 'static,
            $( $ty: FromRequestParts<S> + Send + 'static, )*
            P: OutgoingResponse,
            E: IntoResponse,
            F: Future<Output = Result<P, E>> + Send,
            H: FnOnce($($ty,)* Q) -> F + Clone + Send + 'static,
            S: Debug + Clone + Send + Sync + 'static,
        {
            fn add_to_router(self, router: axum::Router<S>) -> axum::Router<S>
            {
                let meta = Q::METADATA;

                let method = match meta.method {
                    Method::DELETE => MethodFilter::DELETE,
                    Method::GET => MethodFilter::GET,
                    Method::HEAD => MethodFilter::HEAD,
                    Method::OPTIONS => MethodFilter::OPTIONS,
                    Method::PATCH => MethodFilter::PATCH,
                    Method::POST => MethodFilter::POST,
                    Method::PUT => MethodFilter::PUT,
                    Method::TRACE => MethodFilter::TRACE,
                    m => panic!("Unsupported HTTP method: {m:?}"),
                };

                meta.history.all_paths().fold(router, |router, path| {
                    let handler = self.clone();

                    router.route(path, on(method, |$( $ty: $ty, )* request: Incoming<Q>| async move {
                        match handler($( $ty, )* request.0).await {
                            Ok(response) => Outgoing(response).into_response(),
                            Err(error) => error.into_response(),
                        }
                    }))
                })
            }
        }
    };
}

impl_ruma_handler!();
impl_ruma_handler!(T1);
impl_ruma_handler!(T1, T2);
impl_ruma_handler!(T1, T2, T3);
impl_ruma_handler!(T1, T2, T3, T4);
impl_ruma_handler!(T1, T2, T3, T4, T5);
impl_ruma_handler!(T1, T2, T3, T4, T5, T6);
impl_ruma_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_ruma_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
