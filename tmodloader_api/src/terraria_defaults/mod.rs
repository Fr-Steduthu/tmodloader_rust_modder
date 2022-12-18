
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
        use crate::tmod_types::{ItemSoundID, Identifier::Terraria};

        pub const MELEE : ItemSoundID = Terraria(1);
        pub const MUSHROOM : ItemSoundID = Terraria(2);
        pub const POTION : ItemSoundID = Terraria(3);
        pub const CRYSTAL : ItemSoundID = Terraria(4);
        pub const SHOOT : ItemSoundID = Terraria(5);
        pub const MAGIC_MIRROR : ItemSoundID = Terraria(6);
        pub const MELEE_LOWW_PITCHED : ItemSoundID = Terraria(7);
        pub const DEMON_SCYTHE : ItemSoundID = Terraria(8);
        pub const STARFURY : ItemSoundID = Terraria(9);
        pub const HARPOONS : ItemSoundID = Terraria(10);
        pub const GUNS : ItemSoundID = Terraria(11);
        pub const SPACE_GUN : ItemSoundID = Terraria(12);
        pub const WATER_SPRAY : ItemSoundID = Terraria(13);
        pub const EXPLOSION : ItemSoundID = Terraria(14);
        pub const PHASEBLADE : ItemSoundID = Terraria(15);
        pub const WHOPPIE_CUSHION_FART : ItemSoundID = Terraria(16);
        pub const DART : ItemSoundID = Terraria(17);
        pub const MELEE2 : ItemSoundID = Terraria(18);
        pub const THROW : ItemSoundID = Terraria(19);
        pub const FLAMES : ItemSoundID = Terraria(20);
        pub const WATER_BOLT : ItemSoundID = Terraria(21);
        pub const MOTOR_TOOL : ItemSoundID = Terraria(22);
        pub const MOTOR_TOOL_ACTIVATION : ItemSoundID = Terraria(23);
        pub const SPECTRE_BOLT : ItemSoundID = Terraria(24);
        pub const FAIRY_BELL : ItemSoundID = Terraria(25);
        pub const HARP : ItemSoundID = Terraria(26);
        pub const COIN_CLINK : ItemSoundID = Terraria(27);
        pub const RAINBOW_ROD : ItemSoundID = Terraria(28);
        pub const MANA_CRYSTAL : ItemSoundID = Terraria(29);
        pub const ICE_ROD_BLOCK_APPEARENCE : ItemSoundID = Terraria(30);
        pub const ASSAULT_RIFLE : ItemSoundID = Terraria(31);
        pub const WINGS_FLAP : ItemSoundID = Terraria(32);
        pub const MECA_BOSS_LASER : ItemSoundID = Terraria(33);
        pub const FLAMETHROWER : ItemSoundID = Terraria(34);
        pub const BELL : ItemSoundID = Terraria(35);
        pub const SHOTGUN : ItemSoundID = Terraria(36);
        pub const ANVIL_CLINK : ItemSoundID = Terraria(37);
    }
    pub mod entities {
        use crate::tmod_types::{EntitySoundID, Identifier::Terraria};

        pub const DIG: EntitySoundID = Terraria(0); // 1
        pub const PLAYER_KILLED: EntitySoundID = Terraria(5);
        pub const GRASS: EntitySoundID = Terraria(6);
        pub const GRAB: EntitySoundID = Terraria(7);
        pub const DOOR_OPEN: EntitySoundID = Terraria(8);
        pub const DOOR_CLOSED: EntitySoundID = Terraria(9);
        pub const MENU_OPEN: EntitySoundID = Terraria(10);
        pub const MENU_CLOSED: EntitySoundID = Terraria(11);
        pub const MENU_TICK: EntitySoundID = Terraria(12);
        pub const SHATTER: EntitySoundID = Terraria(13);
        //pas de n°14
        pub const ROAR: EntitySoundID = Terraria(15); // 0 -> 2
        pub const DOUBLE_JUMP: EntitySoundID = Terraria(16);
        pub const RUN: EntitySoundID = Terraria(17);
        pub const COINS: EntitySoundID = Terraria(18);
        pub const SPLASH: EntitySoundID = Terraria(19); // 0 -> 5
        pub const FEMALE_HIT: EntitySoundID = Terraria(20); // Player
        pub const TINK: EntitySoundID = Terraria(21);
        pub const UNLOCK: EntitySoundID = Terraria(22);
        pub const DROWN: EntitySoundID = Terraria(23);
        pub const CHAT: EntitySoundID = Terraria(24);
        pub const MAX_MANA: EntitySoundID = Terraria(25);
        pub const MUMMY: EntitySoundID = Terraria(26);
        pub const PIXIE: EntitySoundID = Terraria(27);
        pub const MECH: EntitySoundID = Terraria(28);
        pub const ZOMBIE: EntitySoundID = Terraria(29); // 0 -> 117
        pub const DUCK: EntitySoundID = Terraria(30);
        pub const FROG: EntitySoundID = Terraria(31);
        pub const BIRD: EntitySoundID = Terraria(32);
        pub const CRITTER: EntitySoundID = Terraria(33);
        pub const WATTERFALL: EntitySoundID = Terraria(34);
        pub const LAVAFALL: EntitySoundID = Terraria(35);
        pub const FORCE_ROAR: EntitySoundID = Terraria(36); // EoC Master mode roar
        pub const MEOWMERE: EntitySoundID = Terraria(37);
        pub const COIN_PICKUP: EntitySoundID = Terraria(38);
        pub const DRIP: EntitySoundID = Terraria(39); // 0 -> 2
        pub const CAMERA: EntitySoundID = Terraria(40);
        pub const MOONLORD: EntitySoundID = Terraria(41);
        //42
        pub const THUNDER: EntitySoundID = Terraria(43);
        pub const SEAGULL: EntitySoundID = Terraria(44);
        pub const DOLPHIN: EntitySoundID = Terraria(45);
        pub const OWL: EntitySoundID = Terraria(46);
        pub const GUITAR_C: EntitySoundID = Terraria(47);
        pub const GUITAR_D: EntitySoundID = Terraria(48);
        pub const GUITAR_EM: EntitySoundID = Terraria(49);
        pub const GUITAR_G: EntitySoundID = Terraria(50);
        pub const GUITAR_BM: EntitySoundID = Terraria(51);
        pub const GUITAR_AM: EntitySoundID = Terraria(52);
        pub const DRUM_HI_HAT: EntitySoundID = Terraria(53);
        pub const DRUM_TOM_HIGH: EntitySoundID = Terraria(54);
        pub const DRUM_TOM_LOW: EntitySoundID = Terraria(55);
        pub const DRUM_TOM_MID: EntitySoundID = Terraria(56);
        pub const DRUM_CLOSED_HI_HAT: EntitySoundID = Terraria(57);
        pub const DRUM_CYMBAL_1: EntitySoundID = Terraria(58);
        pub const DRUM_CYMBAL_2: EntitySoundID = Terraria(59);
        pub const DRUM_KICK: EntitySoundID = Terraria(60);
        pub const DRUM_TAMA_SNARE: EntitySoundID = Terraria(61);
        pub const DRUM_FLOOR_TOM: EntitySoundID = Terraria(62);
        pub const RESEARCH: EntitySoundID = Terraria(63);
        pub const RESEARCH_COMPLETE: EntitySoundID = Terraria(64);
        pub const QUEEN_SLIME: EntitySoundID = Terraria(65);
        pub const CLOWN: EntitySoundID = Terraria(66);
        pub const COCKTIEL: EntitySoundID = Terraria(67);
        pub const MACAW: EntitySoundID = Terraria(68);
        pub const TOUCAN: EntitySoundID = Terraria(69);
    }
}

pub mod tile_ids {
    use crate::tmod_types::Identifier::Terraria;
    use crate::tmod_types::TileId;

    pub const DIRT_TILE : TileId = Terraria(1);
}

