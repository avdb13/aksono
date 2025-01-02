use diesel::{result::Error, BelongingToDsl, QueryDsl};
use diesel_async::RunQueryDsl as _;
use ruma::{DeviceId, UserId};

use crate::{
    entities::{access_tokens::AccessToken, devices::Device},
    schema::access_tokens,
    traits::Crud,
    Conn, Result,
};

impl Crud for AccessToken {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(access_tokens::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(access_tokens::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl AccessToken {
    pub async fn create(
        conn: &mut Conn<'_>,
        value: &str,
        user_id: &UserId,
        device_id: Option<&DeviceId>,
    ) -> Result<()> {
        <AccessToken as Crud>::create(
            conn,
            Self {
                id: value.to_owned(),
                user_id: user_id.to_string(),
                device_id: device_id.map(DeviceId::to_string),
                valid_until_ms: None,
                puppets_user_id: None,
                last_validated: None,
                refresh_token_id: None,
                used: None,
            },
        )
        .await?;

        Ok(())
    }

    pub async fn get(conn: &mut Conn<'_>, value: &str) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, value.to_owned()).await {
            Ok(access_token) => Ok(Some(access_token)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn delete(conn: &mut Conn<'_>, value: &str) -> Result<()> {
        <Self as Crud>::delete(conn, value.to_owned()).await?;

        Ok(())
    }

    pub async fn find_by_device(conn: &mut Conn<'_>, id: &DeviceId) -> Result<Option<Vec<Self>>> {
        match Device::get(conn, id).await? {
            Some(device) => Some(Self::belonging_to(&device).load(conn).await).transpose(),
            None => Ok(None),
        }
    }
}