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
pub mod sound_ids {
    use crate::tmod_types::UseSound;
    use crate::tmod_types::UseSound::Terraria;

    const DIG : UseSound = Terraria(0); // 1
    const PlayerHit : UseSound = Terraria(1); // 1
    const IdNo2 : UseSound = Terraria(2); // 1 -> 178
    const NPCHit : UseSound = Terraria(3); // 1 -> 57
    const NPCDeath : UseSound = Terraria(4); // 1 -> 66
    const PlayerKilled : UseSound = Terraria(5);
    const Grass : UseSound = Terraria(6);
    const Grab : UseSound = Terraria(7);
    const DoorOpen : UseSound = Terraria(8);
    const DoorClosed : UseSound = Terraria(9);
    const MenuOpen : UseSound = Terraria(10);
    const MenuClosed : UseSound = Terraria(11);
    const MenuTick : UseSound = Terraria(12);
    const Shatter : UseSound = Terraria(13);
    //pas de n°14
    const Roar : UseSound = Terraria(15); // 0 -> 2
    const DoubleJump : UseSound = Terraria(16);
    const Run : UseSound = Terraria(17);
    const Coins : UseSound = Terraria(18);
    const Splash : UseSound = Terraria(19); // 0 -> 5
    const FemaleHit : UseSound = Terraria(20); // Player
    const Tink : UseSound = Terraria(21);
    const Unlock : UseSound = Terraria(22);
    const Drown : UseSound = Terraria(23);
    const Chat : UseSound = Terraria(24);
    const MaxMana : UseSound = Terraria(25);
    const Mummy : UseSound = Terraria(26);
    const Pixie : UseSound = Terraria(27);
    const Mech : UseSound = Terraria(28);
    const Zombie : UseSound = Terraria(29); // 0 -> 117
    const Duck : UseSound = Terraria(30);
    const Frog : UseSound = Terraria(31);
    const Bird : UseSound = Terraria(32);
    const Critter : UseSound = Terraria(33);
    const Wetterfall : UseSound = Terraria(34);
    const Lavafall : UseSound = Terraria(35);
    const ForceRoar : UseSound = Terraria(36); // EoC Master mode roar
    const Meowmere : UseSound = Terraria(37);
    const CoinPickup : UseSound = Terraria(38);
    const Drip : UseSound = Terraria(39); // 0 -> 2
    const Camera : UseSound = Terraria(40);
    const Moolord : UseSound = Terraria(41);
    //42
    const Thunder : UseSound = Terraria(43);
    const Seagull : UseSound = Terraria(44);
    const Dolphin : UseSound = Terraria(45);
    const Owl : UseSound = Terraria(46);
    const GuitarC : UseSound = Terraria(47);
    const GuitarD : UseSound = Terraria(48);
    const GuitarEm : UseSound = Terraria(49);
    const GuitarG : UseSound = Terraria(50);
    const GuitarBm : UseSound = Terraria(51);
    const GuitarAm : UseSound = Terraria(52);
    const DrumHiHat : UseSound = Terraria(53);
    const DrumTomHigh : UseSound = Terraria(54);
    const DrumTomLow : UseSound = Terraria(55);
    const DrumTomMid : UseSound = Terraria(56);
    const DrumClosedHiHat : UseSound = Terraria(57);
    const DrumCymbal1 : UseSound = Terraria(58);
    const DrumCymbal2 : UseSound = Terraria(59);
    const DrumKick : UseSound = Terraria(60);
    const DrumTamaSnare : UseSound = Terraria(61);
    const DrumFloorTom : UseSound = Terraria(62);
    const Research : UseSound = Terraria(63);
    const ResearchComplete : UseSound = Terraria(64);
    const QueenSlime : UseSound = Terraria(65);
    const Clown : UseSound = Terraria(66);
    const Cocktiel : UseSound = Terraria(67);
    const Macaw : UseSound = Terraria(68);
    const Toucan : UseSound = Terraria(69);
}

lazy_static::lazy_static!(
    static ref DIRT_TILE : TileId =
    Identifier {
        id: "dirt_tile".to_string(),
        is_vanilla: true,
    };
);

