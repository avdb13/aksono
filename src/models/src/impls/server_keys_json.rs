use diesel::QueryDsl;
use diesel_async::RunQueryDsl as _;

use crate::{
    entities::server_keys_json::ServerKeysJson, schema::server_keys_json, traits::Crud, Conn,
    Result,
};

impl Crud for ServerKeysJson {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(server_keys_json::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(server_keys_json::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl ServerKeysJson {
    // pub async fn create(
    //     conn: &mut Conn<'_>,
    //     value: &str,
    //     user_id: &UserId,
    //     device_id: Option<&DeviceId>,
    // ) -> Result<()> {
    //     <Self as Crud>::create(
    //         conn,
    //         Self {
    //             id: value.to_owned(),
    //             user_id: user_id.to_string(),
    //             device_id: device_id.map(DeviceId::to_string),
    //             valid_until_ms: None,
    //             puppets_user_id: None,
    //             last_validated: None,
    //             refresh_token_id: None,
    //             used: None,
    //         },
    //     )
    //     .await?;

    //     Ok(())
    // }

    // pub async fn get(conn: &mut Conn<'_>, value: &str) -> Result<Option<Self>> {
    //     match <Self as Crud>::get(conn, value.to_owned()).await {
    //         Ok(access_token) => Ok(Some(access_token)),
    //         Err(Error::NotFound) => Ok(None),
    //         Err(error) => Err(error),
    //     }
    // }

    // pub async fn delete(conn: &mut Conn<'_>, value: &str) -> Result<()> {
    //     <Self as Crud>::delete(conn, value.to_owned()).await?;

    //     Ok(())
    // }

    // pub async fn find_by_device(conn: &mut Conn<'_>, id: &DeviceId) -> Result<Option<Vec<Self>>> {
    //     match Device::get(conn, id).await? {
    //         Some(device) => Some(Self::belonging_to(&device).load(conn).await).transpose(),
    //         None => Ok(None),
    //     }
    // }
}
