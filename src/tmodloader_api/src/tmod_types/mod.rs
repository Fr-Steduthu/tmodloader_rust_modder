use crate::cs_types::{CSPrimalType, CSType};
use crate::terraria_defaults::time::TICK;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mod {
    name : String,
    id : [char; 3],
    namespaces : Vec<Namespace>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Namespace {
    name : String,
    contents : Vec<NamespaceContents>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum NamespaceContents {
    Item(Item),
    Projectile(Projectile),
    Buff(Buff),
    Tile(Tile),
    Entity(Entity),
    Recipe(Recipe),
}

pub type Time = u32;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Identifier {
    pub id : String,
    pub is_vanilla : bool,
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        if self.is_vanilla {
            return {
                todo!()
            }
        }

        return {
            todo!()
        }
    }
}

pub type ItemId = Identifier;
pub type ProjectileId = Identifier;
pub type TileId = Identifier;

impl Into<CSType> for Identifier {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum EntitySound {
    Terraria(u16),
    Custom(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ItemSound {
    Terraria(u16),
    Custom(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum UseStyle {
    Swing,
    Eat,
    Stab,
    HoldUp,
    HoldOut
}

impl Into<u16> for UseStyle {
    fn into(self) -> u16 {
        match self {
            UseStyle::Swing => { 1 }
            UseStyle::Eat => { 2 }
            UseStyle::Stab => { 3 }
            UseStyle::HoldUp => { 4 }
            UseStyle::HoldOut => { 5 }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Value {
    platinum : u16,
    gold : u16,
    silver : u16,
    copper : u16
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        self.copper as u64 +
            self.silver as u64 * 100 +
            self.gold as u64 * 10000 +
            self.platinum as u64 * 1000000
        as u64
    }
}

impl Into<CSType> for Value {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Rarity {
    Gray,
    White,
    Blue,
    Green,
    Orange,
    LightRed,
    Pink,
    LightPurple,
    Lime,
    Yellow,
    Cyan,
    Red,
    Purple,
    Rainbow,
    FieryRed,
    Quest,
}

impl Into<i8> for Rarity {
    fn into(self) -> i8 {
        match self {
            Rarity::Gray => { -1 }
            Rarity::White => { 0 }
            Rarity::Blue => { 1 }
            Rarity::Green => { 2 }
            Rarity::Orange => { 3 }
            Rarity::LightRed => { 4 }
            Rarity::Pink => { 5 }
            Rarity::LightPurple => { 6 }
            Rarity::Lime => { 7 }
            Rarity::Yellow => { 8 }
            Rarity::Cyan => { 9 }
            Rarity::Red => { 10 }
            Rarity::Purple => { 11 }
            Rarity::Rainbow => { -12 }
            Rarity::FieryRed => { -13 }
            Rarity::Quest => { -11 }
        }
    }
}

impl From<i8> for Rarity {
    fn from(value: i8) -> Self {
        match value {
            -1 => { Rarity::Gray }
            0 => { Rarity::White }
            1 => { Rarity::Blue }
            2 => { Rarity::Green }
            3 => { Rarity::Orange }
            4 => { Rarity::LightRed }
            5 => { Rarity::Pink }
            6 => { Rarity::LightPurple }
            7 => { Rarity::Lime }
            8 => { Rarity::Yellow }
            9 => { Rarity::Cyan }
            10 => { Rarity::Red }
            11 => { Rarity::Purple }
            -12 => { Rarity::Rainbow }
            -13 => { Rarity::FieryRed }
            -11 => { Rarity::Quest }
            _ => { Rarity::Gray }
        }
    }
}

impl Into<CSType> for Rarity {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DamageType {
    Melee,
    Ranged,
    Magic,
    Summon,
    Thrown,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
    id : ItemId,

    name : String,
    tooltip : String,
    value : Value,
    rarity : Rarity,

    max_stack: u64,

    //Sprite
    width : u16,
    height : u16,

    //Animation
    use_time : self::Time,
    use_animation : self::Time,
    use_style : self::UseStyle,
    use_sound : self::ItemSound,

    auto_reuse : bool,
    consumable : bool, // Loses 1 stack per use

    no_use_graphics : bool, // ??

    use_turn : bool, // Sprite rotation when used
    no_melee : bool, // Whether or not the sprite should be used when using

    damage : i64,
    damage_type : DamageType,
    knockback : u16,

    shoot : Option<ProjectileId>,
    shoot_speed : self::Time,
    use_ammo: Option<ItemId>,

    heal_life: u16,
}

impl Item {
    fn new(name : String, tooltip : String) -> Self {
        Item{
            id: Identifier { id: name.clone(), is_vanilla: false },
            name: name,
            tooltip: tooltip,
            value: Value {
                platinum: 0,
                gold: 0,
                silver: 0,
                copper: 0,
            },
            rarity: 0.into(),
            max_stack: 1,
            width: 0,
            height: 0,
            use_time: 20 * TICK,
            use_animation: 20 * TICK,
            use_style: UseStyle::Swing,
            use_sound: crate::terraria_defaults::sound_ids::items::Melee,
            auto_reuse: false,
            consumable: false,
            no_use_graphics: false,
            use_turn: true,
            no_melee: false,
            damage: 1,
            damage_type: DamageType::Melee,
            knockback: 0,
            shoot: None,
            shoot_speed: 0,
            use_ammo: None,
            heal_life: 0,
        }
    }
}

impl Into<CSType> for Item {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Item".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    result : ItemId,
    ingredients : Vec<ItemId>,
    stations : Vec<TileId>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projectile {
    //TODO
}

impl Into<CSType> for Projectile {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Projectile".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tile {
    //TODO
}

impl Into<CSType> for Tile {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Tile".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Buff {
    //TODO
}

impl Into<CSType> for Buff {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Buff".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Entity {
    //TODO
}
impl Into<CSType> for Entity {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Entity".to_string()).into()
    }
}
