use aksono_common::utils;
use diesel::{result::Error, QueryDsl as _};
use diesel_async::RunQueryDsl as _;
use ruma::UserId;

use crate::{entities::users::User, schema::users, traits::Crud, Conn, Result};

impl Crud for User {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(users::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(users::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl User {
    pub async fn create(
        conn: &mut Conn<'_>,
        id: &UserId,
        password_hash: Option<&str>,
        is_guest: bool,
    ) -> Result<()> {
        <Self as Crud>::create(
            conn,
            Self {
                id: id.to_string(),
                password_hash: password_hash.map(str::to_owned),
                is_guest,
                creation_ts: Some(utils::utc_timestamp_millis()),
                admin: false,
                upgrade_ts: None,
                appservice_id: None,
                deactivated: false,
            },
        )
        .await?;

        Ok(())
    }

    pub async fn get(conn: &mut Conn<'_>, id: &UserId) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, id.to_string()).await {
            Ok(user) => Ok(Some(user)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn change_password(
        conn: &mut Conn<'_>,
        id: &UserId,
        password_hash: &str,
    ) -> Result<()> {
        let user = <Self as Crud>::get(conn, id.to_string()).await?;

        <Self as Crud>::update(
            conn,
            id.to_string(),
            Self {
                password_hash: Some(password_hash.to_owned()),
                ..user
            },
        )
        .await?;

        Ok(())
    }
}
