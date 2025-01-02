use aksono_common::utils;
use diesel::{result::Error, QueryDsl};
use diesel_async::RunQueryDsl as _;
use ruma::CanonicalJsonObject;

use crate::{
    entities::uiaa_sessions::UiaaSession, schema::uiaa_sessions, traits::Crud, Conn, Result,
};

impl Crud for UiaaSession {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(uiaa_sessions::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(uiaa_sessions::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl UiaaSession {
    pub async fn create(conn: &mut Conn<'_>, id: &str, json: CanonicalJsonObject) -> Result<()> {
        <Self as Crud>::create(
            conn,
            Self {
                id: id.to_owned(),
                creation_time: utils::utc_timestamp_millis(),
                json: serde_json::to_string(&json).expect("valid CanonicalJsonObject"),
            },
        )
        .await?;

        Ok(())
    }

    pub async fn get(conn: &mut Conn<'_>, id: &str) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, id.to_owned()).await {
            Ok(uiaa_session) => Ok(Some(uiaa_session)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn update(conn: &mut Conn<'_>, id: &str, json: &CanonicalJsonObject) -> Result<()> {
        let value = <Self as Crud>::get(conn, id.to_owned()).await?;

        <Self as Crud>::update(
            conn,
            id.to_owned(),
            UiaaSession {
                json: serde_json::to_string(&json).unwrap(),
                ..value
            },
        )
        .await?;

        Ok(())
    }

    pub async fn delete(conn: &mut Conn<'_>, id: &str) -> Result<()> {
        <Self as Crud>::delete(conn, id.to_owned()).await?;

        Ok(())
    }
}
