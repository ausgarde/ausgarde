#[cfg(feature = "database")]
use postgres_types::FromSql;
#[cfg(feature = "database")]
use tokio_postgres::types::ToSql;

use bytes::BytesMut;
use ulid::Ulid;

#[derive(Debug)]
pub struct UserId(ulid::Ulid);

impl UserId {
    #![allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(ulid::Ulid::new())
    }
}

impl std::str::FromStr for UserId {
    type Err = ulid::DecodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(ulid::Ulid::from_string(value)?))
    }
}

impl<'de> serde::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<UserId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = Ulid::deserialize(deserializer)?;

        Ok(UserId(id))
    }
}

impl serde::Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<B: AsRef<[u8]>> From<B> for UserId {
    fn from(value: B) -> Self {
        let mut bytes = [0u8; 16];
        let reader = value.as_ref();

        assert!(reader.len() <= 16, "Buffer does not have enough bytes");

        bytes.copy_from_slice(&reader[..16]);

        Self(ulid::Ulid::from_bytes(bytes))
    }
}

#[cfg(feature = "database")]
impl<'a> FromSql<'a> for UserId {
    fn from_sql(
        _: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::from(raw))
    }

    fn accepts(_: &postgres_types::Type) -> bool {
        true
    }
}

#[cfg(feature = "database")]
impl ToSql for UserId {
    fn to_sql(
        &self,
        _: &postgres_types::Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        out.extend_from_slice(&self.0.to_bytes());
        Ok(postgres_types::IsNull::No)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        matches!(ty.kind(), postgres_types::Kind::Simple)
    }

    tokio_postgres::types::to_sql_checked!();
}
