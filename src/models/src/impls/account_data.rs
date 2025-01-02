use diesel::QueryDsl as _;
use diesel_async::RunQueryDsl as _;
use ruma::{
    events::{EventContent, GlobalAccountDataEventContent, StaticEventContent},
    UserId,
};

use crate::{
    entities::account_data::AccountData, schema::account_data, traits::Crud, Conn, Result,
};

impl Crud for AccountData {
    type Id = String;

    async fn create(conn: &mut Conn<'_>, values: Self) -> Result<usize> {
        diesel::insert_into(account_data::table)
            .values(values)
            .execute(conn)
            .await
    }

    async fn update(conn: &mut Conn<'_>, id: Self::Id, values: Self) -> Result<usize> {
        diesel::update(account_data::table.find(id))
            .set(values)
            .execute(conn)
            .await
    }
}

impl AccountData {
    pub async fn create<C>(
        conn: &mut Conn<'_>,
        id: &UserId,
        content: &C,
        instance_name: Option<&str>,
    ) -> Result<()>
    where
        C: GlobalAccountDataEventContent + StaticEventContent,
    {
        let kind = <C as EventContent>::EventType::from(<C as StaticEventContent>::TYPE);

        <Self as Crud>::create(
            conn,
            Self {
                id: id.to_string(),
                kind: kind.to_string(),
                content: serde_json::to_string(content).unwrap(),
                instance_name: instance_name.map(str::to_owned),
            },
        )
        .await?;

        Ok(())
    }
}
