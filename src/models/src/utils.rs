use diesel::{
    backend::Backend,
    deserialize::{FromSql, Result},
    sql_types::{SmallInt, Text},
    Queryable,
};
use ruma::CanonicalJsonObject;

#[derive(Debug)]
pub struct BoolInt(i16);

impl From<BoolInt> for bool {
    fn from(n: BoolInt) -> Self {
        n.0 != 0
    }
}

impl<DB> Queryable<SmallInt, DB> for BoolInt
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    type Row = i16;

    fn build(n: i16) -> Result<Self> {
        Ok(BoolInt(n))
    }
}

#[derive(Debug)]
pub struct Json(String);

impl From<Json> for CanonicalJsonObject {
    fn from(json: Json) -> Self {
        serde_json::from_str(&json.0).expect("Json should always be valid")
    }
}

impl<DB> Queryable<Text, DB> for Json
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> Result<Self> {
        Ok(Self(s))
    }
}
