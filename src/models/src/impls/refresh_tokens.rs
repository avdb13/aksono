use diesel::{result::Error, BelongingToDsl, QueryDsl};
use diesel_async::RunQueryDsl as _;
use ruma::{DeviceId, UserId};

use crate::{
    entities::{devices::Device, refresh_tokens::RefreshToken},
    schema::refresh_tokens,
    traits::Crud,
    Conn, Result,
};

impl Crud for RefreshToken {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(refresh_tokens::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(refresh_tokens::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl RefreshToken {
    pub async fn create(
        conn: &mut Conn<'_>,
        value: &str,
        user_id: &UserId,
        device_id: &DeviceId,
        expiry_ts: i64,
    ) -> Result<()> {
        <RefreshToken as Crud>::create(
            conn,
            Self {
                id: value.to_owned(),
                user_id: user_id.to_string(),
                device_id: device_id.to_string(),
                expiry_ts: Some(expiry_ts),
            },
        )
        .await?;

        Ok(())
    }

    pub async fn get(conn: &mut Conn<'_>, value: &str) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, value.to_owned()).await {
            Ok(refresh_token) => Ok(Some(refresh_token)),
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
