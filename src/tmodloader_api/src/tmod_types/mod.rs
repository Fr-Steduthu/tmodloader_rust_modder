use crate::cs_types::{AccessModifier, CSFunction, CSObject, CSPrimalType, CSType};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Mod {
    name : String,
    id : [char; 3],
    namespaces : Vec<Namespace>
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Namespace {
    name : String,
    contents : Vec<NamespaceContents>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum NamespaceContents {
    Item(Item),
    Projectile(Projectile),
    Buff(Buff),
    Tile(Tile),
    Entity(Entity),
    Recipe(Recipe),
}

pub type Time = u32;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum Identifier {
    Terraria(u32),
    Modded(String),
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        match self {
            Identifier::Terraria(id) => { id.to_string() }
            Identifier::Modded(id) => { id.clone() }
        }
    }
}

pub type ItemId = Identifier;
pub type ProjectileId = Identifier;
pub type TileId = Identifier;
pub type BuffId = Identifier;
pub type

impl Into<CSType> for Identifier {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum EntitySound {
    Terraria(u16),
    Custom(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ItemSound {
    Terraria(u16),
    Custom(String),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Value {
    pub platinum : u16,
    pub gold : u16,
    pub silver : u16,
    pub copper : u16
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum DamageType {
    Melee,
    Ranged,
    Magic,
    Summon,
    Thrown,
}

impl ToString for DamageType {
    fn to_string(&self) -> String {
        match self {
            DamageType::Melee => { "melee".to_string() }
            DamageType::Ranged => { "ranged".to_string() }
            DamageType::Magic => { "magic".to_string() }
            DamageType::Summon => { "summon".to_string() }
            DamageType::Thrown => { "thorwn".to_string() }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Item {
    pub id : ItemId,

    pub name : String,
    pub tooltip : String,
    pub value : Value,
    pub rarity : Rarity,

    pub max_stack: u64,

    //Sprite
    pub width : u16,
    pub height : u16,

    //Animation
    pub use_time : self::Time,
    pub use_animation : self::Time,
    pub use_style : self::UseStyle,
    pub use_sound : self::ItemSound,

    pub auto_reuse : bool,
    pub consumable : bool, // Loses 1 stack per use

    pub no_use_graphics : bool, // ??

    pub use_turn : bool, // Sprite rotation when used
    pub no_melee : bool, // Whether or not the sprite should be used when using

    pub damage : i64,
    pub damage_type : Option<DamageType>,
    pub knockback : u16,

    pub shoot : Option<ProjectileId>,
    pub shoot_speed : self::Time,
    pub use_ammo: Option<ItemId>,

    pub heal_life: u16,
}

impl Into<CSType> for Item {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Item".to_string()).into()
    }
}

impl Into<CSObject> for Item {
    fn into(self) -> CSObject {
        CSObject{
            classname: self.name.clone(),
            namespace: "ModItem".to_string(),
            accessibility: AccessModifier::Public,
            inherits : vec!["ModItem".to_string()],
            fields: vec![],
            functions: vec![
                CSFunction {
                    name: "setDefaults".to_string(),
                    access: AccessModifier::Private,
                    arguments: vec![],
                    body: {
                        [
                            "\t\titem.name = ", self.name.as_str(), ";\n",
                            "\t\titem.tooltip = \"", self.tooltip.as_str(), "\";\n",
                            if <Value as Into<u64>>::into(self.value.clone()) != 0u64 { ["\t\titem.value = ", <Value as Into<u64>>::into(self.value.clone()).to_string().as_str(), ";\n"].join("") } else { "".to_string() }.as_str(),
                            if let Some(t) = self.damage_type { ["\t\titem.", t.to_string().as_str(), " = true;\n\t\titem.damage = ", self.damage.to_string().as_str(), "\n"].join("") } else { "".to_string() }.as_str(),
                            if self.use_turn { "\t\tthis.useTurn = true;\n" } else { "" },
                            if self.auto_reuse { "\t\tthis.autoReuse = true;\n"} else { "" },
                        ].join("")
                    },
                    return_value: CSType {
                        prefix: crate::cs_types::CSTypePrefix::None,
                        t: CSPrimalType::Void,
                        is_array: false,
                    },
                    is_override: true,
                    scoped_variables: vec![],
                }
            ],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Recipe {
    result : ItemId,
    ingredients : Vec<ItemId>,
    stations : Vec<TileId>,
}

impl ToString for Recipe {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Projectile {
    id : ProjectileId,
    //TODO
}

impl ToString for Projectile {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Into<CSType> for Projectile {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Projectile".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Tile {
    id : TileId,
    //TODO
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Into<CSType> for Tile {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Tile".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Buff {
    id : BuffId,
    //TODO
}

impl ToString for Buff {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Into<CSType> for Buff {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Buff".to_string()).into()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Entity {
    id : EntityId,
    //TODO
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl Into<CSType> for Entity {
    fn into(self) -> CSType {
        CSPrimalType::Custom("Entity".to_string()).into()
    }
}
