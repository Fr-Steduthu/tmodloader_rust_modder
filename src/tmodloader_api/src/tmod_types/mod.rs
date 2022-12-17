use crate::cs_types::{CSPrimalType, CSType};

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

pub enum UseSound {
    Terraria(u16),
    Custom(Identifier)
}
impl Into<CSType> for u16 {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Value {
    platinum : u16,
    gold : u16,
    silver : u16,
    copper : u16
}

impl Into<CSType> for Value {
    fn into(self) -> CSType {
        CSPrimalType::Integer.into()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Rarity {
    Dull,
    Legendary,
    //TODO
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
    use_time : crate::terraria_defaults::time::Time,
    use_animation : crate::terraria_defaults::time::Time,


    auto_reuse : bool,
    consumable : bool, // Loses 1 stack per use

    no_use_graphics : bool, // ??

    use_turn : bool, // Sprite rotation when used
    no_melee : bool, // Whether or not the sprite should be used when using

    damage : i64,
    damage_type : DamageType,
    knockback : u16,

    shoot : ProjectileId,
    shoot_speed : crate::terraria_defaults::time::Time,

    heal_life: u16,
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
