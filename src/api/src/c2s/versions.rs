use ruma::api::{
    client::discovery::get_supported_versions::{Request, Response},
    MatrixVersion,
};

use crate::Result;

/// `GET /_matrix/client/versions` ([spec])
///
/// [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixclientversions
pub async fn get_supported_versions(_: Request) -> Result<Response> {
    let versions: Vec<_> = (0..=11)
        .map(|m| {
            MatrixVersion::from_parts(1, m)
                .expect("valid MatrixVersions")
                .to_string()
        })
        .collect();

    Ok(Response::new(versions))
}
