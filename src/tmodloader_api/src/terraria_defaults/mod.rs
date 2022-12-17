use std::string::ToString;
use crate::tmod_types::{Identifier, TileId};

pub mod time {
    use crate::tmod_types::Time;

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
    pub mod items {
        use crate::tmod_types::{ItemSound, ItemSound::Terraria};

        pub const Melee : ItemSound = Terraria(1);
        pub const Mushroom : ItemSound = Terraria(2);
        pub const Potion : ItemSound = Terraria(3);
        pub const Crystal : ItemSound = Terraria(4);
        pub const Shoot : ItemSound = Terraria(5);
        pub const MagicMirror : ItemSound = Terraria(6);
        pub const MeleeLowPitch : ItemSound = Terraria(7);
        pub const DemonScythe : ItemSound = Terraria(8);
        pub const StarFury : ItemSound = Terraria(9);
        pub const Harpoon : ItemSound = Terraria(10);
        pub const Guns : ItemSound = Terraria(11);
        pub const SpaceGun : ItemSound = Terraria(12);
        pub const WaterSpray : ItemSound = Terraria(13);
        pub const Explosion : ItemSound = Terraria(14);
        pub const Phaseblade : ItemSound = Terraria(15);
        pub const WhoopieCushionFart : ItemSound = Terraria(16);
        pub const Dart : ItemSound = Terraria(17);
        pub const MeleeBis : ItemSound = Terraria(18);
        pub const Throw : ItemSound = Terraria(19);
        pub const Flames : ItemSound = Terraria(20);
        pub const WaterBolt : ItemSound = Terraria(21);
        pub const MotorTool : ItemSound = Terraria(22);
        pub const MotorToolActivation : ItemSound = Terraria(23);
        pub const SpectreBolt : ItemSound = Terraria(24);
        pub const FairyBell : ItemSound = Terraria(25);
        pub const Harp : ItemSound = Terraria(26);
        pub const CoinClink : ItemSound = Terraria(27);
        pub const RainbowRod : ItemSound = Terraria(28);
        pub const ManaCrystal : ItemSound = Terraria(29);
        pub const IceRodBlockAppearance : ItemSound = Terraria(30);
        pub const AssaultRifle : ItemSound = Terraria(31);
        pub const WingsFlap : ItemSound = Terraria(32);
        pub const MecaBossLaser : ItemSound = Terraria(33);
        pub const Flamethrower : ItemSound = Terraria(34);
        pub const Bell : ItemSound = Terraria(35);
        pub const Shotgun : ItemSound = Terraria(36);
        pub const AnvilClink : ItemSound = Terraria(37);
    }
    pub mod entities {
        use crate::tmod_types::{EntitySound, EntitySound::Terraria};

        pub const Dig: EntitySound = Terraria(0); // 1
        pub const PlayerKilled: EntitySound = Terraria(5);
        pub const Grass: EntitySound = Terraria(6);
        pub const Grab: EntitySound = Terraria(7);
        pub const DoorOpen: EntitySound = Terraria(8);
        pub const DoorClosed: EntitySound = Terraria(9);
        pub const MenuOpen: EntitySound = Terraria(10);
        pub const MenuClosed: EntitySound = Terraria(11);
        pub const MenuTick: EntitySound = Terraria(12);
        pub const Shatter: EntitySound = Terraria(13);
        //pas de n°14
        pub const Roar: EntitySound = Terraria(15); // 0 -> 2
        pub const DoubleJump: EntitySound = Terraria(16);
        pub const Run: EntitySound = Terraria(17);
        pub const Coins: EntitySound = Terraria(18);
        pub const Splash: EntitySound = Terraria(19); // 0 -> 5
        pub const FemaleHit: EntitySound = Terraria(20); // Player
        pub const Tink: EntitySound = Terraria(21);
        pub const Unlock: EntitySound = Terraria(22);
        pub const Drown: EntitySound = Terraria(23);
        pub const Chat: EntitySound = Terraria(24);
        pub const MaxMana: EntitySound = Terraria(25);
        pub const Mummy: EntitySound = Terraria(26);
        pub const Pixie: EntitySound = Terraria(27);
        pub const Mech: EntitySound = Terraria(28);
        pub const Zombie: EntitySound = Terraria(29); // 0 -> 117
        pub const Duck: EntitySound = Terraria(30);
        pub const Frog: EntitySound = Terraria(31);
        pub const Bird: EntitySound = Terraria(32);
        pub const Critter: EntitySound = Terraria(33);
        pub const Wetterfall: EntitySound = Terraria(34);
        pub const Lavafall: EntitySound = Terraria(35);
        pub const ForceRoar: EntitySound = Terraria(36); // EoC Master mode roar
        pub const Meowmere: EntitySound = Terraria(37);
        pub const CoinPickup: EntitySound = Terraria(38);
        pub const Drip: EntitySound = Terraria(39); // 0 -> 2
        pub const Camera: EntitySound = Terraria(40);
        pub const Moolord: EntitySound = Terraria(41);
        //42
        pub const Thunder: EntitySound = Terraria(43);
        pub const Seagull: EntitySound = Terraria(44);
        pub const Dolphin: EntitySound = Terraria(45);
        pub const Owl: EntitySound = Terraria(46);
        pub const GuitarC: EntitySound = Terraria(47);
        pub const GuitarD: EntitySound = Terraria(48);
        pub const GuitarEm: EntitySound = Terraria(49);
        pub const GuitarG: EntitySound = Terraria(50);
        pub const GuitarBm: EntitySound = Terraria(51);
        pub const GuitarAm: EntitySound = Terraria(52);
        pub const DrumHiHat: EntitySound = Terraria(53);
        pub const DrumTomHigh: EntitySound = Terraria(54);
        pub const DrumTomLow: EntitySound = Terraria(55);
        pub const DrumTomMid: EntitySound = Terraria(56);
        pub const DrumClosedHiHat: EntitySound = Terraria(57);
        pub const DrumCymbal1: EntitySound = Terraria(58);
        pub const DrumCymbal2: EntitySound = Terraria(59);
        pub const DrumKick: EntitySound = Terraria(60);
        pub const DrumTamaSnare: EntitySound = Terraria(61);
        pub const DrumFloorTom: EntitySound = Terraria(62);
        pub const Research: EntitySound = Terraria(63);
        pub const ResearchComplete: EntitySound = Terraria(64);
        pub const QueenSlime: EntitySound = Terraria(65);
        pub const Clown: EntitySound = Terraria(66);
        pub const Cocktiel: EntitySound = Terraria(67);
        pub const Macaw: EntitySound = Terraria(68);
        pub const Toucan: EntitySound = Terraria(69);
    }
}

lazy_static::lazy_static!(
    static ref DIRT_TILE : TileId =
    Identifier {
        id: "dirt_tile".to_string(),
        is_vanilla: true,
    };
);

