use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use bytes::BytesMut;
use ruma::api::OutgoingResponse;

#[derive(Clone)]
pub(crate) struct Outgoing<T>(pub(super) T);

impl<T: OutgoingResponse> IntoResponse for Outgoing<T> {
    fn into_response(self) -> Response {
        match self.0.try_into_http_response::<BytesMut>() {
            Ok(response) => response
                .map(BytesMut::freeze)
                .map(Body::from)
                .into_response(),
            Err(_) => todo!(),
        }
    }
}
