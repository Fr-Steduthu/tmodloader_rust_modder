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

        const Melee : ItemSound = Terraria(1);
        const Mushroom : ItemSound = Terraria(2);
        const Potion : ItemSound = Terraria(3);
        const Crystal : ItemSound = Terraria(4);
        const Shoot : ItemSound = Terraria(5);
        const MagicMirror : ItemSound = Terraria(6);
        const MeleeLowPitch : ItemSound = Terraria(7);
        const DemonScythe : ItemSound = Terraria(8);
        const StarFury : ItemSound = Terraria(9);
        const Harpoon : ItemSound = Terraria(10);
        const Guns : ItemSound = Terraria(11);
        const SpaceGun : ItemSound = Terraria(12);
        const WaterSpray : ItemSound = Terraria(13);
        const Explosion : ItemSound = Terraria(14);
        const Phaseblade : ItemSound = Terraria(15);
        const WhoopieCushionFart : ItemSound = Terraria(16);
        const Dart : ItemSound = Terraria(17);
        const MeleeBis : ItemSound = Terraria(18);
        const Throw : ItemSound = Terraria(19);
        const Flames : ItemSound = Terraria(20);
        const WaterBolt : ItemSound = Terraria(21);
        const MotorTool : ItemSound = Terraria(22);
        const MotorToolActivation : ItemSound = Terraria(23);
        const SpectreBolt : ItemSound = Terraria(24);
        const FairyBell : ItemSound = Terraria(25);
        const Harp : ItemSound = Terraria(26);
        const CoinClink : ItemSound = Terraria(27);
        const RainbowRod : ItemSound = Terraria(28);
        const ManaCrystal : ItemSound = Terraria(29);
        const IceRodBlockAppearance : ItemSound = Terraria(30);
        const AssaultRifle : ItemSound = Terraria(31);
        const WingsFlap : ItemSound = Terraria(32);
        const MecaBossLaser : ItemSound = Terraria(33);
        const Flamethrower : ItemSound = Terraria(34);
        const Bell : ItemSound = Terraria(35);
        const Shotgun : ItemSound = Terraria(36);
        const AnvilClink : ItemSound = Terraria(37);
    }
    pub mod entities {
        use crate::tmod_types::{EntitySound, EntitySound::Terraria};

        const Dig: EntitySound = Terraria(0); // 1
        const PlayerHit: EntitySound = Terraria(1); // 1
        const IdNo2: EntitySound = Terraria(2); // 1 -> 178
        const NPCHit: EntitySound = Terraria(3); // 1 -> 57
        const NPCDeath: EntitySound = Terraria(4); // 1 -> 66
        const PlayerKilled: EntitySound = Terraria(5);
        const Grass: EntitySound = Terraria(6);
        const Grab: EntitySound = Terraria(7);
        const DoorOpen: EntitySound = Terraria(8);
        const DoorClosed: EntitySound = Terraria(9);
        const MenuOpen: EntitySound = Terraria(10);
        const MenuClosed: EntitySound = Terraria(11);
        const MenuTick: EntitySound = Terraria(12);
        const Shatter: EntitySound = Terraria(13);
        //pas de n°14
        const Roar: EntitySound = Terraria(15); // 0 -> 2
        const DoubleJump: EntitySound = Terraria(16);
        const Run: EntitySound = Terraria(17);
        const Coins: EntitySound = Terraria(18);
        const Splash: EntitySound = Terraria(19); // 0 -> 5
        const FemaleHit: EntitySound = Terraria(20); // Player
        const Tink: EntitySound = Terraria(21);
        const Unlock: EntitySound = Terraria(22);
        const Drown: EntitySound = Terraria(23);
        const Chat: EntitySound = Terraria(24);
        const MaxMana: EntitySound = Terraria(25);
        const Mummy: EntitySound = Terraria(26);
        const Pixie: EntitySound = Terraria(27);
        const Mech: EntitySound = Terraria(28);
        const Zombie: EntitySound = Terraria(29); // 0 -> 117
        const Duck: EntitySound = Terraria(30);
        const Frog: EntitySound = Terraria(31);
        const Bird: EntitySound = Terraria(32);
        const Critter: EntitySound = Terraria(33);
        const Wetterfall: EntitySound = Terraria(34);
        const Lavafall: EntitySound = Terraria(35);
        const ForceRoar: EntitySound = Terraria(36); // EoC Master mode roar
        const Meowmere: EntitySound = Terraria(37);
        const CoinPickup: EntitySound = Terraria(38);
        const Drip: EntitySound = Terraria(39); // 0 -> 2
        const Camera: EntitySound = Terraria(40);
        const Moolord: EntitySound = Terraria(41);
        //42
        const Thunder: EntitySound = Terraria(43);
        const Seagull: EntitySound = Terraria(44);
        const Dolphin: EntitySound = Terraria(45);
        const Owl: EntitySound = Terraria(46);
        const GuitarC: EntitySound = Terraria(47);
        const GuitarD: EntitySound = Terraria(48);
        const GuitarEm: EntitySound = Terraria(49);
        const GuitarG: EntitySound = Terraria(50);
        const GuitarBm: EntitySound = Terraria(51);
        const GuitarAm: EntitySound = Terraria(52);
        const DrumHiHat: EntitySound = Terraria(53);
        const DrumTomHigh: EntitySound = Terraria(54);
        const DrumTomLow: EntitySound = Terraria(55);
        const DrumTomMid: EntitySound = Terraria(56);
        const DrumClosedHiHat: EntitySound = Terraria(57);
        const DrumCymbal1: EntitySound = Terraria(58);
        const DrumCymbal2: EntitySound = Terraria(59);
        const DrumKick: EntitySound = Terraria(60);
        const DrumTamaSnare: EntitySound = Terraria(61);
        const DrumFloorTom: EntitySound = Terraria(62);
        const Research: EntitySound = Terraria(63);
        const ResearchComplete: EntitySound = Terraria(64);
        const QueenSlime: EntitySound = Terraria(65);
        const Clown: EntitySound = Terraria(66);
        const Cocktiel: EntitySound = Terraria(67);
        const Macaw: EntitySound = Terraria(68);
        const Toucan: EntitySound = Terraria(69);
    }
}

lazy_static::lazy_static!(
    static ref DIRT_TILE : TileId =
    Identifier {
        id: "dirt_tile".to_string(),
        is_vanilla: true,
    };
);

