//! JSON columns read as raw text, so a store can hand them to its caller without parsing them.

use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use sqlx::{
    Postgres,
    error::BoxDynError,
    postgres::{PgTypeInfo, PgValueRef},
    types::Json,
};

/// A JSON column read as its raw text, passed through to the caller unparsed.
#[derive(Clone, Debug)]
pub struct RawJson(pub Box<RawValue>);

impl sqlx::Type<Postgres> for RawJson {
    fn type_info() -> PgTypeInfo {
        <Json<Box<RawValue>> as sqlx::Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <Json<Box<RawValue>> as sqlx::Type<Postgres>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for RawJson {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        <Json<Box<RawValue>> as sqlx::Decode<'r, Postgres>>::decode(value).map(|json| Self(json.0))
    }
}

impl PartialEq for RawJson {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl RawJson {
    /// The column's JSON text exactly as Postgres returned it.
    pub fn get(&self) -> &str {
        self.0.get()
    }
}

impl Serialize for RawJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
