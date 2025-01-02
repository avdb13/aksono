mod schema;

pub mod entities;
pub mod impls;
pub mod traits;
pub mod utils;

pub(crate) type Conn<'c> =
    diesel_async::pooled_connection::bb8::PooledConnection<'c, diesel_async::AsyncPgConnection>;

pub(crate) type Result<T> = core::result::Result<T, diesel::result::Error>;
