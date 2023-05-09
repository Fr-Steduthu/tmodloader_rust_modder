
pub mod time
{
    use crate::tmod_types::Time;

    pub const TICK: Time = 1;

    const SECOND_IRL: Time = 60;
    const MINUTE_IRL: Time = SECOND_IRL * 60;
    const HOUR_IRL: Time = MINUTE_IRL * 60;

    const SECOND_IG: Time = TICK;
    const MINUTE_IG: Time = SECOND_IRL;
    const HOUR_IG: Time = MINUTE_IRL;

    pub const DAYTIME: (Time, Time) = (4 * HOUR_IG + 30 * MINUTE_IG, 7 * HOUR_IG + 30 * MINUTE_IG);
    pub const NIGHTTIME: (Time, Time) = (7 * HOUR_IG + 30 * MINUTE_IG + SECOND_IG, 4 * HOUR_IG + 30 * MINUTE_IG - SECOND_IG);

    pub mod irl
    {
        use crate::terraria::time::{HOUR_IRL, MINUTE_IRL, SECOND_IRL};
        use crate::tmod_types::Time;

        pub fn seconds(amount: u32) -> Time
        {
            amount * SECOND_IRL
        }
        pub fn minutes(amount: u32) -> Time
        {
            amount * MINUTE_IRL
        }
        pub fn hours(amount: u32) -> Time
        {
            amount * HOUR_IRL
        }
    }

    pub mod ig
    {
        use crate::terraria::time::{HOUR_IG, MINUTE_IG, SECOND_IG};
        use crate::tmod_types::Time;

        pub fn seconds(amount: u32) -> Time
        {
            amount * SECOND_IG
        }
        pub fn minutes(amount: u32) -> Time
        {
            amount * MINUTE_IG
        }
        pub fn hours(amount: u32) -> Time
        {
            amount * HOUR_IG
        }
    }
}

macro_rules! id
{
    ($name: ident, $t: ty, $val: literal) => {
        #[allow(non_snake_case)] pub fn $name() -> $t {
            <$t>::from(Terraria($val.to_string()))
        }
    };
}

pub mod sound_ids {
    pub mod items {
        use crate::tmod_types::{Identifier::Terraria, ItemSoundId};

        id!(Melee, ItemSoundId, "SoundID.Item1");
        // pub const MUSHROOM : ItemSoundId = ItemSoundId(Terraria(2));
        // pub const POTION : ItemSoundId = ItemSoundId(Terraria(3));
        // pub const CRYSTAL : ItemSoundId = ItemSoundId(Terraria(4));
        // pub const SHOOT : ItemSoundId = ItemSoundId(Terraria(5));
        // pub const MAGIC_MIRROR : ItemSoundId = ItemSoundId(Terraria(6));
        // pub const MELEE_LOWW_PITCHED : ItemSoundId = ItemSoundId(Terraria(7));
        // pub const DEMON_SCYTHE : ItemSoundId = ItemSoundId(Terraria(8));
        // pub const STARFURY : ItemSoundId = ItemSoundId(Terraria(9));
        // pub const HARPOONS : ItemSoundId = ItemSoundId(Terraria(10));
        // pub const GUNS : ItemSoundId = ItemSoundId(Terraria(11));
        // pub const SPACE_GUN : ItemSoundId = ItemSoundId(Terraria(12));
        // pub const WATER_SPRAY : ItemSoundId = ItemSoundId(Terraria(13));
        // pub const EXPLOSION : ItemSoundId = ItemSoundId(Terraria(14));
        // pub const PHASEBLADE : ItemSoundId = ItemSoundId(Terraria(15));
        // pub const WHOPPIE_CUSHION_FART : ItemSoundId = ItemSoundId(Terraria(16));
        // pub const DART : ItemSoundId = ItemSoundId(Terraria(17));
        // pub const MELEE2 : ItemSoundId = ItemSoundId(Terraria(18));
        // pub const THROW : ItemSoundId = ItemSoundId(Terraria(19));
        // pub const FLAMES : ItemSoundId = ItemSoundId(Terraria(20));
        // pub const WATER_BOLT : ItemSoundId = ItemSoundId(Terraria(21));
        // pub const MOTOR_TOOL : ItemSoundId = ItemSoundId(Terraria(22));
        // pub const MOTOR_TOOL_ACTIVATION : ItemSoundId = ItemSoundId(Terraria(23));
        // pub const SPECTRE_BOLT : ItemSoundId = ItemSoundId(Terraria(24));
        // pub const FAIRY_BELL : ItemSoundId = ItemSoundId(Terraria(25));
        // pub const HARP : ItemSoundId = ItemSoundId(Terraria(26));
        // pub const COIN_CLINK : ItemSoundId = ItemSoundId(Terraria(27));
        // pub const RAINBOW_ROD : ItemSoundId = ItemSoundId(Terraria(28));
        // pub const MANA_CRYSTAL : ItemSoundId = ItemSoundId(Terraria(29));
        // pub const ICE_ROD_BLOCK_APPEARENCE : ItemSoundId = ItemSoundId(Terraria(30));
        // pub const ASSAULT_RIFLE : ItemSoundId = ItemSoundId(Terraria(31));
        // pub const WINGS_FLAP : ItemSoundId = ItemSoundId(Terraria(32));
        // pub const MECA_BOSS_LASER : ItemSoundId = ItemSoundId(Terraria(33));
        // pub const FLAMETHROWER : ItemSoundId = ItemSoundId(Terraria(34));
        // pub const BELL : ItemSoundId = ItemSoundId(Terraria(35));
        // pub const SHOTGUN : ItemSoundId = ItemSoundId(Terraria(36));
        // pub const ANVIL_CLINK : ItemSoundId = ItemSoundId(Terraria(37));
    }
    /*pub mod entities {
        use crate::tmod_types::{EntitySoundId, Identifier::Terraria};


        const fn id(s: &str) -> EntitySoundId
        {
            EntitySoundId(Terraria(String::from(s)))
        }

        pub const DIG: EntitySoundId = EntitySoundId(Terraria(0)); // 1
        pub const PLAYER_KILLED: EntitySoundId = EntitySoundId(Terraria(5));
        pub const GRASS: EntitySoundId = EntitySoundId(Terraria(6));
        pub const GRAB: EntitySoundId = EntitySoundId(Terraria(7));
        pub const DOOR_OPEN: EntitySoundId = EntitySoundId(Terraria(8));
        pub const DOOR_CLOSED: EntitySoundId = EntitySoundId(Terraria(9));
        pub const MENU_OPEN: EntitySoundId = EntitySoundId(Terraria(10));
        pub const MENU_CLOSED: EntitySoundId = EntitySoundId(Terraria(11));
        pub const MENU_TICK: EntitySoundId = EntitySoundId(Terraria(12));
        pub const SHATTER: EntitySoundId = EntitySoundId(Terraria(13));
        //pas de n°14
        pub const ROAR: EntitySoundId = EntitySoundId(Terraria(15)); // 0 -> 2
        pub const DOUBLE_JUMP: EntitySoundId = EntitySoundId(Terraria(16));
        pub const RUN: EntitySoundId = EntitySoundId(Terraria(17));
        pub const COINS: EntitySoundId = EntitySoundId(Terraria(18));
        pub const SPLASH: EntitySoundId = EntitySoundId(Terraria(19)); // 0 -> 5
        pub const FEMALE_HIT: EntitySoundId = EntitySoundId(Terraria(20)); // Player
        pub const TINK: EntitySoundId = EntitySoundId(Terraria(21));
        pub const UNLOCK: EntitySoundId = EntitySoundId(Terraria(22));
        pub const DROWN: EntitySoundId = EntitySoundId(Terraria(23));
        pub const CHAT: EntitySoundId = EntitySoundId(Terraria(24));
        pub const MAX_MANA: EntitySoundId = EntitySoundId(Terraria(25));
        pub const MUMMY: EntitySoundId = EntitySoundId(Terraria(26));
        pub const PIXIE: EntitySoundId = EntitySoundId(Terraria(27));
        pub const MECH: EntitySoundId = EntitySoundId(Terraria(28));
        pub const ZOMBIE: EntitySoundId = EntitySoundId(Terraria(29)); // 0 -> 117
        pub const DUCK: EntitySoundId = EntitySoundId(Terraria(30));
        pub const FROG: EntitySoundId = EntitySoundId(Terraria(31));
        pub const BIRD: EntitySoundId = EntitySoundId(Terraria(32));
        pub const CRITTER: EntitySoundId = EntitySoundId(Terraria(33));
        pub const WATTERFALL: EntitySoundId = EntitySoundId(Terraria(34));
        pub const LAVAFALL: EntitySoundId = EntitySoundId(Terraria(35));
        pub const FORCE_ROAR: EntitySoundId = EntitySoundId(Terraria(36)); // EoC Master mode roar
        pub const MEOWMERE: EntitySoundId = EntitySoundId(Terraria(37));
        pub const COIN_PICKUP: EntitySoundId = EntitySoundId(Terraria(38));
        pub const DRIP: EntitySoundId = EntitySoundId(Terraria(39)); // 0 -> 2
        pub const CAMERA: EntitySoundId = EntitySoundId(Terraria(40));
        pub const MOONLORD: EntitySoundId = EntitySoundId(Terraria(41));
        //42
        pub const THUNDER: EntitySoundId = EntitySoundId(Terraria(43));
        pub const SEAGULL: EntitySoundId = EntitySoundId(Terraria(44));
        pub const DOLPHIN: EntitySoundId = EntitySoundId(Terraria(45));
        pub const OWL: EntitySoundId = EntitySoundId(Terraria(46));
        pub const GUITAR_C: EntitySoundId = EntitySoundId(Terraria(47));
        pub const GUITAR_D: EntitySoundId = EntitySoundId(Terraria(48));
        pub const GUITAR_EM: EntitySoundId = EntitySoundId(Terraria(49));
        pub const GUITAR_G: EntitySoundId = EntitySoundId(Terraria(50));
        pub const GUITAR_BM: EntitySoundId = EntitySoundId(Terraria(51));
        pub const GUITAR_AM: EntitySoundId = EntitySoundId(Terraria(52));
        pub const DRUM_HI_HAT: EntitySoundId = EntitySoundId(Terraria(53));
        pub const DRUM_TOM_HIGH: EntitySoundId = EntitySoundId(Terraria(54));
        pub const DRUM_TOM_LOW: EntitySoundId = EntitySoundId(Terraria(55));
        pub const DRUM_TOM_MID: EntitySoundId = EntitySoundId(Terraria(56));
        pub const DRUM_CLOSED_HI_HAT: EntitySoundId = EntitySoundId(Terraria(57));
        pub const DRUM_CYMBAL_1: EntitySoundId = EntitySoundId(Terraria(58));
        pub const DRUM_CYMBAL_2: EntitySoundId = EntitySoundId(Terraria(59));
        pub const DRUM_KICK: EntitySoundId = EntitySoundId(Terraria(60));
        pub const DRUM_TAMA_SNARE: EntitySoundId = EntitySoundId(Terraria(61));
        pub const DRUM_FLOOR_TOM: EntitySoundId = EntitySoundId(Terraria(62));
        pub const RESEARCH: EntitySoundId = EntitySoundId(Terraria(63));
        pub const RESEARCH_COMPLETE: EntitySoundId = EntitySoundId(Terraria(64));
        pub const QUEEN_SLIME: EntitySoundId = EntitySoundId(Terraria(65));
        pub const CLOWN: EntitySoundId = EntitySoundId(Terraria(66));
        pub const COCKTIEL: EntitySoundId = EntitySoundId(Terraria(67));
        pub const MACAW: EntitySoundId = EntitySoundId(Terraria(68));
        pub const TOUCAN: EntitySoundId = EntitySoundId(Terraria(69));
    }*/
}

pub mod tile_ids {
    use crate::tmod_types::Identifier::Terraria;
    use crate::tmod_types::TileId;

// Reference: https://terraria.fandom.com/wiki/Tile_IDs#cite_ref-1

    id!(Dirt, TileId, "0");
    id!(Stone, TileId, "1");
    id!(Grass, TileId, "2");
    id!(Flowers, TileId, "3");
    id!(Torches, TileId, "4");
    id!(Trees, TileId, "5");
    id!(IronOre, TileId, "6");
    id!(CopperOre, TileId, "7");
    id!(GoldOre, TileId, "8");
    id!(SilverOre, TileId, "9");
    id!(ClosedDoors, TileId, "10");
    id!(OpenDoors, TileId, "11");
    id!(LifeCrystal, TileId, "12");
    id!(Bottles, TileId, "13");
    id!(Tables, TileId, "14");
    id!(Chairs, TileId, "15");
    id!(Anvils, TileId, "16");
    id!(Furnace, TileId, "17");
    id!(Workbenches, TileId, "18");
    id!(Platforms, TileId, "19");
    id!(Saplings, TileId, "20");
    id!(Chests, TileId, "21");
    id!(DemoniteOre, TileId, "22");
    id!(CorruptGrass, TileId, "23");
    id!(CorruptionPlant, TileId, "24");
    id!(EbonStone, TileId, "25");
    id!(DemonAltar, TileId, "26");
    id!(Sunflower, TileId, "27");
    id!(Pots, TileId, "28");
    id!(Piggybank, TileId, "29");
    id!(Wood, TileId, "30");
}

pub mod item_ids
{
    use crate::tmod_types::
    {
        Identifier::Terraria, ItemId
    };

    id!(IronPickaxe, ItemId, "ItemID.IronPickaxe");
    id!(DirtBlock, ItemId, "ItemID.DirtBlock");
}
