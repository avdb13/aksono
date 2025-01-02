use diesel::{
    result::Error, BelongingToDsl, QueryDsl as _,
};
use diesel_async::RunQueryDsl as _;
use ruma::{DeviceId, UserId};

use crate::{
    entities::{devices::Device, users::User},
    schema::devices,
    traits::Crud,
    Conn, Result,
};

impl Crud for Device {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(devices::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(devices::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl Device {
    pub async fn get(conn: &mut Conn<'_>, id: &DeviceId) -> Result<Option<Self>> {
        match <Self as Crud>::get(conn, id.to_string()).await {
            Ok(device) => Ok(Some(device)),
            Err(Error::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn create(
        conn: &mut Conn<'_>,
        id: &DeviceId,
        user_id: &UserId,
        display_name: Option<&str>,
    ) -> Result<()> {
        <Self as Crud>::create(
            conn,
            Self {
                id: id.to_string(),
                user_id: user_id.to_string(),
                display_name: display_name.map(str::to_owned),
                last_seen: None,
                hidden: None,
            },
        )
        .await?;

        Ok(())
    }

    pub async fn delete(conn: &mut Conn<'_>, id: &DeviceId) -> Result<()> {
        <Self as Crud>::delete(conn, id.to_string()).await?;

        Ok(())
    }

    pub async fn find_by_user(conn: &mut Conn<'_>, id: &UserId) -> Result<Option<Vec<Self>>> {
        match User::get(conn, id).await? {
            Some(user) => Some(Self::belonging_to(&user).load(conn).await).transpose(),
            None => Ok(None),
        }
    }
}
