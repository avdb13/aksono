use diesel::{result::Error, BelongingToDsl, QueryDsl};
use diesel_async::RunQueryDsl as _;
use ruma::api::client::uiaa::AuthType;

use crate::{
    entities::{uiaa_credentials::UiaaCredentials, uiaa_sessions::UiaaSession},
    schema::uiaa_credentials,
    traits::Crud,
    Conn, Result,
};

impl Crud for UiaaCredentials {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(uiaa_credentials::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(uiaa_credentials::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl UiaaCredentials {
    pub async fn create(
        conn: &mut Conn<'_>,
        id: &str,
        stage_type: &AuthType,
        result: (),
    ) -> Result<()> {
        <UiaaCredentials as Crud>::create(
            conn,
            Self {
                id: id.to_owned(),
                stage_type: stage_type.to_string(),
                result: todo!(),
            },
        )
        .await?;

        Ok(())
    }

    pub async fn get(conn: &mut Conn<'_>, id: &str) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, id.to_owned()).await {
            Ok(uiaa_credentials) => Ok(Some(uiaa_credentials)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn delete(conn: &mut Conn<'_>, id: &str) -> Result<()> {
        <Self as Crud>::delete(conn, id.to_owned()).await?;

        Ok(())
    }

    pub async fn find_by_session(conn: &mut Conn<'_>, id: &str) -> Result<Option<Vec<Self>>> {
        match UiaaSession::get(conn, id).await? {
            Some(session) => Some(Self::belonging_to(&session).load(conn).await).transpose(),
            None => Ok(None),
        }
    }
}
