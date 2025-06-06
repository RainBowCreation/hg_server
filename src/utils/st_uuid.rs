use uuid::Uuid;
use serde::{Serialize, Deserialize as SerdeDeserialize};
use std::{fmt::Display, ops::Deref};
use spacetimedb::{
    sats::{impl_deserialize, impl_serialize, impl_st},
    ReducerContext,
};

#[derive(Debug, Serialize, SerdeDeserialize)]
pub struct StUuid(pub Uuid);

// spacetimedb impls
impl_st!([] StUuid, spacetimedb::sats::AlgebraicType::String);
impl_serialize!([] StUuid, (self, ser) => {
    ser.serialize_str(self.hyphenated().encode_upper(&mut Uuid::encode_buffer()))
});
impl_deserialize!([] StUuid, de => {
    let s = <std::string::String as spacetimedb::Deserialize>::deserialize(de).map(|s| s.into_boxed_str())?;
    Ok(Uuid::parse_str(&s).map(|u| u.into()).expect("Failed to Deserialize to UUID"))
});

impl StUuid {
    pub fn new(ctx: &ReducerContext) -> Self {
        StUuid(Uuid::from_bytes(ctx.random()))
    }
}

impl Display for StUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Deref for StUuid {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for StUuid {
    fn from(val: Uuid) -> Self {
        Self(val)
    }
}