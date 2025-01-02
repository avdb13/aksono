use aksono_common::{Error, Result};
use diesel::QueryDsl as _;
use diesel_async::RunQueryDsl as _;

use crate::{
    entities::registration_token::RegistrationToken,
    schema::registration_tokens::dsl::registration_tokens, Conn,
};

impl RegistrationToken {
    pub async fn find_by_token(conn: &mut Conn<'_>, token: &str) -> Result<Option<Self>> {
        match registration_tokens.find(token).first(conn).await {
            Ok(registration_token) => Ok(Some(registration_token)),
            Err(error) => match error {
                diesel::result::Error::NotFound => Ok(None),
                error => Err(Error::Diesel { source: error }),
            },
        }
    }
}
