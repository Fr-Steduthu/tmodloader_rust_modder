use std::string::ToString;
use crate::tmod_types::{Identifier, TileId};

pub mod time {
    pub type Time = u32;
    pub const TICK: Time = 1;

    pub const SECOND_IRL: Time = 60;
    pub const MINUTE_IRL: Time = SECOND_IRL * 60;
    pub const HOUR_IRL: Time = MINUTE_IRL * 60;

    pub const SECOND_IG: Time = TICK;
    pub const MINUTE_IG: Time = SECOND_IRL;
    pub const HOUR_IG: Time = MINUTE_IRL;

    pub const DAYTIME: (Time, Time) = (4 * HOUR_IG + 30 * MINUTE_IG, 7 * HOUR_IG + 30 * MINUTE_IG);
    pub const NIGHTTIME: (Time, Time) = (7 * HOUR_IG + 30 * MINUTE_IG + SECOND_IG, 4 * HOUR_IG + 30 * MINUTE_IG - SECOND_IG);
}

lazy_static::lazy_static!(
    static ref DIRT_TILE : TileId =
    Identifier {
        id: "dirt_tile".to_string(),
        is_vanilla: true,
    };
);

