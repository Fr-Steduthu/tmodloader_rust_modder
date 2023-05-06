use std::fmt::{Display, Formatter};
use crate::concat_cs_code;
use crate::cs::CSTemplate;

pub type Time = u32;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Mod {
    pub(crate) name : String,
    pub(crate) display_name: String,
    pub(crate) author: String,
    pub(crate) version: String,
    pub(crate) description: String,

    //pub(crate) id : [char; 3],
    pub(crate) contents: Vec<ModFolder>
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
            contents: vec![],
        }
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
        for folders in self.contents
        {
            todo!("Export files in ModContents")
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ModFolder {
    name : String,
    contents : Vec<ModFolderContents>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ModFolderContents {
    Item(Item),
    Projectile(Projectile),
    Buff(Buff),
    Tile(Tile),
    Entity(Entity),
    Recipe(Recipe),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum Identifier {
    Terraria(u32),
    Modded(String),
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

pub type ItemId = Identifier;
pub type ProjectileId = Identifier;
pub type TileId = Identifier;
pub type BuffId = Identifier;
pub type EntityId = Identifier;

pub type EntitySoundID = Identifier;
pub type ItemSoundID = Identifier;

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

    pub name : String,
    pub tooltip : String,
    pub value : Value,
    pub rarity : Rarity,

    pub max_stack: u64,

    //Sprite
    pub width : u16,
    pub height : u16,

    //Animation
    pub use_time : Time,
    pub use_animation : Time,
    pub use_style : UseStyle,
    pub use_sound : ItemSoundID,

    pub auto_reuse : bool,
    pub consumable : bool, // Loses 1 stack per use

    pub no_use_graphics : bool, // ??

    pub use_turn : bool, // Sprite rotation when used
    pub no_melee : bool, // Whether or not the sprite should be used when using

    pub damage : i64,
    pub damage_type : Option<DamageType>,
    pub knockback : u16,

    pub shoot : Option<ProjectileId>,
    pub shoot_speed : Time,
    pub use_ammo: Option<ItemId>,

    pub heal_life: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Recipe {
    result : ItemId,
    ingredients : Vec<ItemId>,
    stations : Vec<TileId>,
}

impl CSTemplate for Recipe
{
    fn to_cs(self) -> String
    {
        let mut s = String::new();

        s.push_str("\tpublic override void AddRecipes()\n\r\t{\n\r\t\tModRecipe recipe = new ModRecipe(mod);\n\r");

        for ingr in self.ingredients
        {
            s.push_str("\t\trecipe.AddIngredient(ItemID.DirtBlock, 10);\n\r");
        }

        for tile in self.stations
        {
            s.push_str("\t\trecipe.AddTile(TileID.WorkBenches);\n\r");
        }

        s.push_str(concat_cs_code!("\t\trecipe.SetResult(", self.result, ");\n\r").as_str());

        s.push_str("\t\trecipe.AddRecipe();\n\r\t}\n\r");

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
