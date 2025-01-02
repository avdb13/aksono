use std::future::Future;

use diesel::{
    associations::HasTable,
    dsl,
    query_builder::{DeleteStatement, IntoUpdateTarget},
    query_dsl::methods::{FindDsl, LimitDsl},
    result::Error,
};
use diesel_async::{
    methods::{ExecuteDsl, LoadQuery},
    AsyncPgConnection, RunQueryDsl,
};

use crate::Conn;

pub(crate) trait Crud: HasTable + Send + Sized
where
    Self::Table: FindDsl<Self::Id>,
    dsl::Find<<Self as HasTable>::Table, <Self as Crud>::Id>: LimitDsl + IntoUpdateTarget + Send,
    DeleteStatement<
        <dsl::Find<<Self as HasTable>::Table, <Self as Crud>::Id> as HasTable>::Table,
        <dsl::Find<<Self as HasTable>::Table, <Self as Crud>::Id> as IntoUpdateTarget>::WhereClause,
    >: ExecuteDsl<AsyncPgConnection> + Send + 'static,
    dsl::Limit<dsl::Find<<Self as HasTable>::Table, <Self as Crud>::Id>>:
        LoadQuery<'static, AsyncPgConnection, Self> + Send + 'static,
{
    type Id: Send;

    fn create(
        conn: &mut Conn<'_>,
        values: Self,
    ) -> impl Future<Output = Result<usize, Error>> + Send;

    fn get(conn: &mut Conn<'_>, id: Self::Id) -> impl Future<Output = Result<Self, Error>> {
        async move { Self::table().find(id).first(conn).await }
    }

    /// when you want to null out a column, you have to send Some(None)), since sending None means you
    /// just don't want to update that column.
    fn update(
        conn: &mut Conn<'_>,
        id: Self::Id,
        values: Self,
    ) -> impl Future<Output = Result<usize, Error>>;

    fn delete(conn: &mut Conn<'_>, id: Self::Id) -> impl Future<Output = Result<usize, Error>> {
        async move { diesel::delete(Self::table().find(id)).execute(conn).await }
    }
}
