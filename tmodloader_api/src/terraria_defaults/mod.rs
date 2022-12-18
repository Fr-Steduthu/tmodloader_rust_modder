use std::borrow::ToOwned;
use lazy_static::lazy_static;
use csharp_repr::types::CSClass;

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

        pub const MELEE : ItemSound = Terraria(1);
        pub const MUSHROOM : ItemSound = Terraria(2);
        pub const POTION : ItemSound = Terraria(3);
        pub const CRYSTAL : ItemSound = Terraria(4);
        pub const SHOOT : ItemSound = Terraria(5);
        pub const MAGIC_MIRROR : ItemSound = Terraria(6);
        pub const MELEE_LOWW_PITCHED : ItemSound = Terraria(7);
        pub const DEMON_SCYTHE : ItemSound = Terraria(8);
        pub const STARFURY : ItemSound = Terraria(9);
        pub const HARPOONS : ItemSound = Terraria(10);
        pub const GUNS : ItemSound = Terraria(11);
        pub const SPACE_GUN : ItemSound = Terraria(12);
        pub const WATER_SPRAY : ItemSound = Terraria(13);
        pub const EXPLOSION : ItemSound = Terraria(14);
        pub const PHASEBLADE : ItemSound = Terraria(15);
        pub const WHOPPIE_CUSHION_FART : ItemSound = Terraria(16);
        pub const DART : ItemSound = Terraria(17);
        pub const MELEE2 : ItemSound = Terraria(18);
        pub const THROW : ItemSound = Terraria(19);
        pub const FLAMES : ItemSound = Terraria(20);
        pub const WATER_BOLT : ItemSound = Terraria(21);
        pub const MOTOR_TOOL : ItemSound = Terraria(22);
        pub const MOTOR_TOOL_ACTIVATION : ItemSound = Terraria(23);
        pub const SPECTRE_BOLT : ItemSound = Terraria(24);
        pub const FAIRY_BELL : ItemSound = Terraria(25);
        pub const HARP : ItemSound = Terraria(26);
        pub const COIN_CLINK : ItemSound = Terraria(27);
        pub const RAINBOW_ROD : ItemSound = Terraria(28);
        pub const MANA_CRYSTAL : ItemSound = Terraria(29);
        pub const ICE_ROD_BLOCK_APPEARENCE : ItemSound = Terraria(30);
        pub const ASSAULT_RIFLE : ItemSound = Terraria(31);
        pub const WINGS_FLAP : ItemSound = Terraria(32);
        pub const MECA_BOSS_LASER : ItemSound = Terraria(33);
        pub const FLAMETHROWER : ItemSound = Terraria(34);
        pub const BELL : ItemSound = Terraria(35);
        pub const SHOTGUN : ItemSound = Terraria(36);
        pub const ANVIL_CLINK : ItemSound = Terraria(37);
    }
    pub mod entities {
        use crate::tmod_types::{EntitySound, EntitySound::Terraria};

        pub const DIG: EntitySound = Terraria(0); // 1
        pub const PLAYER_KILLED: EntitySound = Terraria(5);
        pub const GRASS: EntitySound = Terraria(6);
        pub const GRAB: EntitySound = Terraria(7);
        pub const DOOR_OPEN: EntitySound = Terraria(8);
        pub const DOOR_CLOSED: EntitySound = Terraria(9);
        pub const MENU_OPEN: EntitySound = Terraria(10);
        pub const MENU_CLOSED: EntitySound = Terraria(11);
        pub const MENU_TICK: EntitySound = Terraria(12);
        pub const SHATTER: EntitySound = Terraria(13);
        //pas de n°14
        pub const ROAR: EntitySound = Terraria(15); // 0 -> 2
        pub const DOUBLE_JUMP: EntitySound = Terraria(16);
        pub const RUN: EntitySound = Terraria(17);
        pub const COINS: EntitySound = Terraria(18);
        pub const SPLASH: EntitySound = Terraria(19); // 0 -> 5
        pub const FEMALE_HIT: EntitySound = Terraria(20); // Player
        pub const TINK: EntitySound = Terraria(21);
        pub const UNLOCK: EntitySound = Terraria(22);
        pub const DROWN: EntitySound = Terraria(23);
        pub const CHAT: EntitySound = Terraria(24);
        pub const MAX_MANA: EntitySound = Terraria(25);
        pub const MUMMY: EntitySound = Terraria(26);
        pub const PIXIE: EntitySound = Terraria(27);
        pub const MECH: EntitySound = Terraria(28);
        pub const ZOMBIE: EntitySound = Terraria(29); // 0 -> 117
        pub const DUCK: EntitySound = Terraria(30);
        pub const FROG: EntitySound = Terraria(31);
        pub const BIRD: EntitySound = Terraria(32);
        pub const CRITTER: EntitySound = Terraria(33);
        pub const WATTERFALL: EntitySound = Terraria(34);
        pub const LAVAFALL: EntitySound = Terraria(35);
        pub const FORCE_ROAR: EntitySound = Terraria(36); // EoC Master mode roar
        pub const MEOWMERE: EntitySound = Terraria(37);
        pub const COIN_PICKUP: EntitySound = Terraria(38);
        pub const DRIP: EntitySound = Terraria(39); // 0 -> 2
        pub const CAMERA: EntitySound = Terraria(40);
        pub const MOONLORD: EntitySound = Terraria(41);
        //42
        pub const THUNDER: EntitySound = Terraria(43);
        pub const SEAGULL: EntitySound = Terraria(44);
        pub const DOLPHIN: EntitySound = Terraria(45);
        pub const OWL: EntitySound = Terraria(46);
        pub const GUITAR_C: EntitySound = Terraria(47);
        pub const GUITAR_D: EntitySound = Terraria(48);
        pub const GUITAR_EM: EntitySound = Terraria(49);
        pub const GUITAR_G: EntitySound = Terraria(50);
        pub const GUITAR_BM: EntitySound = Terraria(51);
        pub const GUITAR_AM: EntitySound = Terraria(52);
        pub const DRUM_HI_HAT: EntitySound = Terraria(53);
        pub const DRUM_TOM_HIGH: EntitySound = Terraria(54);
        pub const DRUM_TOM_LOW: EntitySound = Terraria(55);
        pub const DRUM_TOM_MID: EntitySound = Terraria(56);
        pub const DRUM_CLOSED_HI_HAT: EntitySound = Terraria(57);
        pub const DRUM_CYMBAL_1: EntitySound = Terraria(58);
        pub const DRUM_CYMBAL_2: EntitySound = Terraria(59);
        pub const DRUM_KICK: EntitySound = Terraria(60);
        pub const DRUM_TAMA_SNARE: EntitySound = Terraria(61);
        pub const DRUM_FLOOR_TOM: EntitySound = Terraria(62);
        pub const RESEARCH: EntitySound = Terraria(63);
        pub const RESEARCH_COMPLETE: EntitySound = Terraria(64);
        pub const QUEEN_SLIME: EntitySound = Terraria(65);
        pub const CLOWN: EntitySound = Terraria(66);
        pub const COCKTIEL: EntitySound = Terraria(67);
        pub const MACAW: EntitySound = Terraria(68);
        pub const TOUCAN: EntitySound = Terraria(69);
    }
}

pub mod tile_ids {
    use crate::tmod_types::Identifier::Terraria;
    use crate::tmod_types::TileId;

    pub const DIRT_TILE : TileId = Terraria(1);
}

lazy_static! {
    pub static ref MODITEM : CSClass = CSClass {
        classname: "ModItem".to_owned(),
        namespace: "".to_owned(),
        accessibility: csharp_repr::types::AccessModifier::Public,
        parents: vec![],
        fields: vec![],
        functions: vec![],
    };
}

