use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::concat_cs_code;
use crate::cs::CSTemplate;
use crate::tmod_types::Identifier::Terraria;
use crate::terraria_defaults::time::TICK;

pub type Time = u32;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Mod {
    pub(crate) name : String,
    pub(crate) display_name: String,
    pub(crate) author: String,
    pub(crate) version: String,
    pub(crate) description: String,

    //pub(crate) id : [char; 3],
    pub(crate) items: Vec<Item>,
}

impl Mod
{
    pub fn init(name: &str, display_name: &str, author: &str, version: &str, desc: &str) -> Self
    {
        Mod
        {
            name: name.to_string(),
            display_name: display_name.to_string(),
            author: author.to_string(),
            version: version.to_string(),
            description: desc.to_string(),
            //id: [],
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: Item)
    {
        self.items.push(item)
    }


    pub fn export(self, project_path: &str)
    {
        Mod::export_build_txt(
            self.display_name,
            self.author,
            self.version
        );
        Mod::export_description(self.description);

        Mod::export_launch_settings();
        // /icon.png

        Mod::export_mod_file(self.name.clone());

        Mod::export_csproj(self.name);

        // /<namespaces>/*.*
        for item in self.items
        {
            //item.export();
            todo!("Export files in item folder")
        }
    }
}

// Exports
impl Mod
{
    fn export_mod_file(backend_name: String)
    {
        let txt = concat_cs_code!("\
        using Terraria.ModLoader;\n\r\
        \n\r\
        namespace ", backend_name.to_string(),"\n\r\
        {\n\r\
            public class ", backend_name.to_string()," : Mod\n\r\
            {\n\r\
            }\n\r\
        }\
        ");

        todo!("Save file to /<ModName>.cs")
    }

    fn export_build_txt(display_name: String, author: String, version: String)
    {
        let txt = concat_cs_code!(
            "displayName = ", display_name, "\n\r\
            author = ", author, "\n\r\
            version = ", version
        );

        todo!("Save the file to /build.txt")
    }

    fn export_description(description: String)
    {
        let txt = description;

        todo!("Save the file to /description.txt")
    }

    fn export_launch_settings()
    {
        let txt = "\
        {\n\r\
          \"profiles\": {\n\r\
            \"Terraria\": {\n\r\
              \"commandName\": \"Executable\",\n\r\
              \"executablePath\": \"dotnet\",\n\r\
              \"commandLineArgs\": \"$(tMLPath)\",\n\r\
              \"workingDirectory\": \"$(tMLSteamPath)\"\n\r\
            },\n\r\
            \"TerrariaServer\": {\n\r\
              \"commandName\": \"Executable\",\n\r\
              \"executablePath\": \"dotnet\",\n\r\
              \"commandLineArgs\": \"$(tMLServerPath)\",\n\r\
              \"workingDirectory\": \"$(tMLSteamPath)\"\n\r\
            }
          }
        }\
        ".to_string();

        todo!("Save the file to /Properties/launchSettings.json")
    }

    fn export_csproj(backend_name: String)
    {
        let txt = concat_cs_code!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\r\
            <Project Sdk=\"Microsoft.NET.Sdk\">\n\r\
              <Import Project=\"..\tModLoader.targets\" />\n\r\
              <PropertyGroup>\n\r\
                <AssemblyName>", backend_name, "</AssemblyName>\n\r\
                <TargetFramework>net6.0</TargetFramework>\n\r\
                <PlatformTarget>AnyCPU</PlatformTarget>\n\r\
                <LangVersion>latest</LangVersion>\n\r\
              </PropertyGroup>\n\r\
              <ItemGroup>\n\r\
                <PackageReference Include=\"tModLoader.CodeAssist\" Version=\"0.1.*\" />
              </ItemGroup>
            </Project>"
        );
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub enum Identifier {
    Terraria(u32),
    Modded(String),
}

impl Identifier {
    pub fn terraria(self)
    {
        unimplemented!("Identifer::terraria(self)")
    }

    pub fn modded(self) -> String
    {
        match self
        {
            Identifier::Terraria(_) => panic!("Cannot match terraria identifier to modded identifier"),
            Identifier::Modded(v) => v,
        }
    }
}

impl Display for Identifier
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Identifier::Terraria(id) => write!(f, "{id}"),
            Identifier::Modded(id) => write!(f, "{id}"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub struct ItemId(pub(crate) Identifier);

impl CSTemplate for ItemId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }

        return self.0.modded()
    }
}

impl From<Identifier> for ItemId
{
    fn from(value: Identifier) -> Self
    {
        ItemId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ProjectileId(pub(crate) Identifier);

impl CSTemplate for ProjectileId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the ProjectileId(Modded(str))")
    }
}

impl From<Identifier> for ProjectileId
{
    fn from(value: Identifier) -> Self
    {
        ProjectileId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub struct TileId(pub(crate) Identifier);

impl CSTemplate for TileId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the TileId(Modded(str))")
    }
}

impl From<Identifier> for TileId
{
    fn from(value: Identifier) -> Self
    {
        TileId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BuffId(pub(crate) Identifier);

impl CSTemplate for BuffId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the BuffId(Modded(str))")
    }
}

impl From<Identifier> for BuffId
{
    fn from(value: Identifier) -> Self
    {
        BuffId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct EntityId(pub(crate) Identifier);

impl CSTemplate for EntityId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the EntityId(Modded(str))")
    }
}

impl From<Identifier> for EntityId
{
    fn from(value: Identifier) -> Self
    {
        EntityId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct EntitySoundId(pub(crate) Identifier);

impl CSTemplate for EntitySoundId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the EntitySoundID(Modded(str))")
    }
}

impl From<Identifier> for EntitySoundId
{
    fn from(value: Identifier) -> Self
    {
        EntitySoundId(value)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ItemSoundId(pub(crate) Identifier);

impl CSTemplate for ItemSoundId
{
    fn to_cs(self) -> String
    {
        if let Terraria(tid) = self.0
        {
            return tid.to_string();
        }
        todo!("Implement to_cs() for the ItemSoundId(Modded(str))")
    }
}

impl From<Identifier> for ItemSoundId
{
    fn from(value: Identifier) -> Self
    {
        ItemSoundId(value)
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum UseStyle {
    Swing,
    Eat,
    Stab,
    HoldUp,
    HoldOut
}

impl From<UseStyle> for u16
{
    fn from(value: UseStyle) -> Self
    {
        match value {
            UseStyle::Swing => { 1 }
            UseStyle::Eat => { 2 }
            UseStyle::Stab => { 3 }
            UseStyle::HoldUp => { 4 }
            UseStyle::HoldOut => { 5 }
        }
    }
}

impl CSTemplate for UseStyle
{
    fn to_cs(self) -> String
    {
        u16::from(self).to_cs()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Value {
    pub platinum : u16,
    pub gold : u16,
    pub silver : u16,
    pub copper : u16
}

impl From<Value> for u64
{
    fn from(value: Value) -> Self
    {
        value.copper as u64 +
            value.silver as u64 * 100 +
            value.gold as u64 * 10000 +
            value.platinum as u64 * 1000000
        as u64
    }
}

impl CSTemplate for Value
{
    fn to_cs(self) -> String
    {
        u64::from(self).to_string()
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

impl From<Rarity> for i8 {
    fn from(value: Rarity) -> Self {
        match value {
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

impl CSTemplate for Rarity
{
    fn to_cs(self) -> String
    {
        i8::from(self).to_cs()
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

impl CSTemplate for DamageType
{
    fn to_cs(self) -> String
    {
        match self {
            DamageType::Melee => { "this.melee = true".to_string() }
            DamageType::Ranged => { "this.ranged = true".to_string() }
            DamageType::Magic => { "this.magic = true".to_string() }
            DamageType::Summon => { "this.summon = true".to_string() }
            DamageType::Thrown => { "this.thrown = true".to_string() }
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Item {
    pub(crate) id : ItemId,

    pub(crate) name : String,
    pub(crate) tooltip : String,
    pub(crate) value : Value,
    pub(crate) rarity : Rarity,

    pub max_stack: u64,

    //Sprite
    pub(crate) width : u16,
    pub(crate) height : u16,

    //Animation
    pub(crate) use_time : Time,
    pub(crate) use_animation : Time,
    pub(crate) use_style : UseStyle,
    pub(crate) use_sound : ItemSoundId,

    pub(crate) auto_reuse : bool,
    pub(crate) consumable : bool, // Loses 1 stack per use

    no_use_graphics : bool, // ??

    pub(crate) use_turn : bool, // Sprite rotation when used
    pub(crate) no_melee : bool, // Whether or not the sprite should be used when using

    pub(crate) damage : i64,
    pub(crate) damage_type : Option<DamageType>, // TODO : transformer en vec
    pub(crate) knockback : u16,

    pub(crate) shoot : Option<ProjectileId>,
    pub(crate) shoot_speed : Time,
    pub(crate) use_ammo: Option<ItemId>,

    recipes: Vec<Recipe>,

    heal_life: u16,
}

impl Item // Getters
{
    pub fn id(&self) -> &ItemId
    {
        &self.id
    }
}

impl Item // Setters
{
    pub fn add_recipe(&mut self, ingredients: Vec<ItemId>, stations: Vec<TileId>)
    {
        self.recipes.push(
            Recipe
            {
                result: self.id().clone(),
                ingredients: ingredients,
                stations: stations,
            }
        )
    }

    pub fn set_defaults(
        &mut self,
        name: String,
        tooltip: String,
        value: Value,
        rarity: Rarity
    )
    {
        self.name = name;
        self.tooltip = tooltip;
        self.value = value;
        self.rarity = rarity;
    }

    pub fn set_sprite(&mut self)
    {
        todo!("Set the width, height and sprite")
    }

    pub fn set_style(
        &mut self,
        use_time: Time,
        use_animation: Time,
        use_style: UseStyle,
        use_sound: ItemSoundId,
        consumable: bool,
        auto_reuse: bool,
        turn_sprite_on_use: bool,
        use_sprite_hitbox: bool
    )
    {
        self.use_time = use_time;
        self.use_animation = use_animation;
        self.use_style = use_style;
        self.use_sound = use_sound;
        self.consumable = consumable;
        self.auto_reuse = auto_reuse;
        self.use_turn = turn_sprite_on_use;
        self.no_melee = use_sprite_hitbox;
    }

    pub fn set_projectile(
        &mut self,
        projectile: ProjectileId,
        speed: Time,
        ammunition: ItemId
    )
    {
        todo!("Implement Item::set_proejctile")
    }

    pub fn set_damage(&mut self, amount: i64, damage_type: DamageType, knockback: u16)
    {
        self.damage = amount;
        self.damage_type = Some(damage_type);
        self.knockback = knockback;
    }
}

impl Item // Templates
{
    pub fn new(name : String, tooltip : String) -> Self
    {
        Item{
            id: Identifier::Modded(name.clone()).into(),
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
            use_time: 0,
            use_animation: 0,
            use_style: UseStyle::Swing,
            use_sound: crate::terraria_defaults::sound_ids::items::MELEE,
            auto_reuse: false,
            consumable: false,
            no_use_graphics: false,
            use_turn: false,
            no_melee: false,
            damage: 1,
            damage_type: Some(DamageType::Melee),
            knockback: 0,
            shoot: None,
            shoot_speed: 0,
            use_ammo: None,
            recipes: vec![],
            heal_life: 0,
        }
    }

    pub fn usable(name : String, tooltip : String) -> Self
    {
        let mut item = Self::new(name, tooltip);

        item.use_time = 20 * TICK;
        item.use_animation = 20 * TICK;

        item.width = 20;
        item.height = 20;

        return item;
    }

    pub fn consumable(name : String, tooltip : String) -> Self
    {
        let mut item = Self::usable(name, tooltip);

        item.consumable = true;

        return item;
    }

    pub fn weapon(name : String, tooltip : String, damage : i64, damage_type : DamageType) -> Self
    {
        let mut item = Self::usable(name, tooltip);

        item.damage = damage;
        item.damage_type = Some(damage_type);

        return item;
    }

    pub fn sword(name: String, tooltip : String, damage : i64) -> Self
    {
        let mut item = Self::weapon(
            name,
            tooltip,
            damage,
            DamageType::Melee
        );

        item.use_turn = true;

        return item;
    }
}

impl Item // Export
{
    pub fn export(self, path: String)
    {
        self.to_cs();

        todo!("Save to file <path>/<ItemName>.cs")
    }
}

impl CSTemplate for Item
{
    fn to_cs(self) -> String
    {
        let mut class = crate::concat_cs_code!(
            "public class ", self.name.clone(), " : ModItem \n\r\
            {\n\r\
            \tpublic override void SetDefaults()\n\r\
            \t{\n\r\
            \t\titem.name = \"", self.name, "\";\n\r\
            \t\titem.toolTip = \"", self.tooltip, "\";\n\r\
            \n\r\
            \t\titem.damage = ", self.damage, ";\n\r\
            \t\titem.knockBack = ", self.knockback, ";\n\r\
            \t\t", self.damage_type.expect("Optional damage type not supported yet").to_cs() ,"\
            \n\r\
            \t\titem.consumable = ", self.consumable, ";\n\r\
            \t\titem.maxStack = ", self.max_stack, ";\n\r\
            \n\r\
            \t\titem.autoReuse = ", self.auto_reuse, ";\n\r\
            \t\titem.value = ", self.value, ";\n\r\
            \t\titem.rare = ", self.rarity, ";\n\r\
            \n\r\
            \t\titem.width = ", self.width, ";\n\r\
            \t\titem.height = ", self.height, ";\n\r\
            \n\r\
            \t\titem.useTime = ", self.use_time, ";\n\r\
            \t\titem.useAnimation = ", self.use_animation, ";\n\r\
            \t\titem.useStyle = ", self.use_style, ";\n\r\
            \t\titem.useSound = ", self.use_sound, ";\n\r\
            \t}\n\r\
            \n\r",
            {
                if self.recipes.len() != 0 {
                    let mut s = String::new();

                    s.push_str("\tpublic override void AddRecipes()\n\r\
                    \t{\n\r");

                    for recipe in self.recipes
                    {
                        s.push_str(recipe.to_cs(true).as_str());
                    }

                    s.push_str("\t}\n\r");

                    s
                }
                else { "".to_string() }
                .as_str()
            },
            "}"
        );

        class
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Recipe {
    result : ItemId,
    ingredients : Vec<ItemId>,
    stations : Vec<TileId>,
}

impl Recipe
{
    fn to_cs(self, set_result_to_this: bool) -> String
    {
        let mut s = String::new();

        s.push_str("\t\tModRecipe recipe");

        let id = { // Unique recipe name
            let mut h = DefaultHasher::new();
            self.stations.hash(&mut h);
            self.ingredients.hash(&mut h);
            let id = h.finish().to_string();
            s.push_str(&id);
            id
        };

        s.push_str(" = new ModRecipe(mod);\n\r");

        for ingr in self.ingredients
        {
            s.push_str("\t\trecipe.AddIngredient(ItemID.DirtBlock, 10);\n\r");
        }

        for tile in self.stations
        {
            s.push_str("\t\trecipe.AddTile(TileID.WorkBenches);\n\r");
        }

        s.push_str(concat_cs_code!("\t\trecipe.SetResult(",
            if set_result_to_this { "this".to_string() } else { self.result.to_cs() }
            , ");\n\r"
        ).as_str());

        s.push_str("\t\trecipe");
        s.push_str(&id.to_string());
        s.push_str(".AddRecipe();\n\r");

        s
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Projectile {
    id : ProjectileId,
    // TODO
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Tile {
    id : TileId,
    // TODO
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Buff {
    id : BuffId,
    // TODO
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Entity {
    id : EntityId,
    // TODO
}
