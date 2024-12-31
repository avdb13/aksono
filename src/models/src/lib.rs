pub(crate) type Conn<'c> =
    diesel_async::pooled_connection::bb8::PooledConnection<'c, diesel_async::AsyncPgConnection>;
