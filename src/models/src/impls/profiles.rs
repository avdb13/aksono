use diesel::QueryDsl as _;
use diesel_async::RunQueryDsl as _;

use crate::{entities::profiles::Profile, schema::profiles, traits::Crud, Conn, Result};

impl Crud for Profile {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(profiles::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(profiles::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl Profile {
    pub async fn create(conn: &mut Conn<'_>, profile: Self) -> Result<()> {
        <Self as Crud>::create(conn, profile).await?;

        Ok(())
    }
}
