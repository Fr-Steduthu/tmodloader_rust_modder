use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{ErrorKind, Write};
use crate::concat_cs_code;
use crate::cs::IntoCSCode;
use crate::tmod_types::Identifier::Terraria;
use crate::terraria::time::TICK;

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

        let project_path = {
            let mut s = project_path.to_string();
            s.push_str("/");
            s.push_str(self.name.as_str());
            s
        };

        match std::fs::create_dir(&project_path)
        {
            Ok(_) => Ok(()),
            Err(v) =>
                {
                    if v.kind() == ErrorKind::AlreadyExists
                    {
                        Ok(())
                    }
                    else
                    {
                        Err(v)
                    }
                },
        }.expect("Could not create project folder");

        Mod::export_build_txt(
            &project_path,
            self.display_name,
            self.author,
            self.version
        );
        Mod::export_description(&project_path, self.description);

        Mod::export_launch_settings(&project_path);
        // /icon.png

        Mod::export_mod_file(&project_path, self.name.clone());

        Mod::export_csproj(&project_path, self.name.clone());

        // /<namespaces>/*.*
        {
            let item_folder_path = {
                let mut s = project_path.clone();
                s.push_str("/Items");
                s
            };
            match std::fs::create_dir(item_folder_path.clone())
            {
                Ok(_) => Ok(()),
                Err(v) =>
                    {
                        if v.kind() == ErrorKind::AlreadyExists
                        {
                            Ok(())
                        } else {
                            Err(v)
                        }
                    }
            }.expect("Could not create items folder");
            for item in self.items
            {
                item.export(item_folder_path.clone(), &self.name.clone());
            }
        }
    }
}

// Exports
impl Mod
{
    fn export_mod_file(path: &str, backend_name: String)
    {
        let txt = concat_cs_code!("\
        using Terraria.ModLoader;\n\
        \n\
        namespace ", backend_name.to_string(),"\n\
        {\n\
            public class ", backend_name.to_string()," : Mod\n\
            {\n\
            }\n\
        }\
        ");

        let mut fd : File = File::create({
            let mut s = path.to_string();
            s.push_str("/");
            s.push_str(&backend_name);
            s.push_str(".cs");
            s
        }).expect("Could not create mod file");
        fd.write(txt.as_bytes()).expect("Could not write to file");
    }

    fn export_build_txt(path: &str, display_name: String, author: String, version: String)
    {
        let txt = concat_cs_code!(
            "displayName = ", display_name, "\n\
            author = ", author, "\n\
            version = ", version
        );

        let mut fd : File = File::create({
            let mut s = path.to_string();
            s.push_str("/build.txt");
            s
        }).expect("Could not create mod file");
        fd.write(txt.as_bytes()).expect("Could not write to file");
    }

    fn export_description(path: &str, description: String)
    {
        let txt = description;

        let mut fd : File = File::create({
            let mut s = path.to_string();
            s.push_str("/description.txt");
            s
        }).expect("Could not create mod file");
        fd.write(txt.as_bytes()).expect("Could not write to file");
    }

    fn export_launch_settings(path: &str)
    {
        let txt = "\
        {\n\
          \"profiles\": {\n\
            \"Terraria\": {\n\
              \"commandName\": \"Executable\",\n\
              \"executablePath\": \"dotnet\",\n\
              \"commandLineArgs\": \"$(tMLPath)\",\n\
              \"workingDirectory\": \"$(tMLSteamPath)\"\n\
            },\n\
            \"TerrariaServer\": {\n\
              \"commandName\": \"Executable\",\n\
              \"executablePath\": \"dotnet\",\n\
              \"commandLineArgs\": \"$(tMLServerPath)\",\n\
              \"workingDirectory\": \"$(tMLSteamPath)\"\n\
            }
          }
        }\
        ".to_string();

        let mut fd : File = File::create({
            let mut s = path.to_string();

            match std::fs::create_dir({
                let mut s = path.to_string();
                s.push_str("/Properties");
                s
            }) {
                Ok(_) => Ok(()),
                Err(v) =>
                    {
                        if v.kind() == ErrorKind::AlreadyExists
                        {
                            Ok(())
                        }
                        else
                        {
                            Err(v)
                        }
                    },
            }.expect("Could not create \"/Properties\" directory");
            s.push_str("/Properties/launchSettings.json");
            s
        }).expect("Could not create mod file");
        fd.write(txt.as_bytes()).expect("Could not write to file");
    }

    fn export_csproj(path: &str, backend_name: String)
    {
        let txt = concat_cs_code!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
            <Project Sdk=\"Microsoft.NET.Sdk\">\n\
              <Import Project=\"..\tModLoader.targets\" />\n\
              <PropertyGroup>\n\
                <AssemblyName>", backend_name, "</AssemblyName>\n\
                <TargetFramework>net6.0</TargetFramework>\n\
                <PlatformTarget>AnyCPU</PlatformTarget>\n\
                <LangVersion>latest</LangVersion>\n\
              </PropertyGroup>\n\
              <ItemGroup>\n\
                <PackageReference Include=\"tModLoader.CodeAssist\" Version=\"0.1.*\" />
              </ItemGroup>
            </Project>"
        );

        let mut fd : File = File::create({
            let mut s = path.to_string();
            s.push_str("/");
            s.push_str(&backend_name);
            s.push_str(".csproj");
            s
        }).expect("Could not create mod file");
        fd.write(txt.as_bytes()).expect("Could not write to file");
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub enum Identifier {
    Terraria(String),
    Modded(String),
}

impl Display for Identifier
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Identifier::Terraria(id) |
            Identifier::Modded(id) => write!(f, "{id}"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Hash)]
pub struct ItemId(pub(crate) Identifier);

impl IntoCSCode for ItemId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string()
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

impl IntoCSCode for ProjectileId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string()
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

impl TileId
{
    fn into_cs(self, mod_name: String) -> String
    {
        match self.0
        {
            Terraria(t_id) => t_id.to_string(),
            Identifier::Modded(m_id) => {
                let mut s = String::new();
                s.push_str(&mod_name);
                s.push_str(&vec!["\"", &mod_name, "\""].join(""));
                s.push_str(&m_id);
                s
            },
        }
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

impl IntoCSCode for BuffId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string()
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

impl IntoCSCode for EntityId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string()
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

impl IntoCSCode for EntitySoundId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string()
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

impl IntoCSCode for ItemSoundId
{
    fn into_cs(self) -> String
    {
        return self.0.to_string();
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

impl IntoCSCode for UseStyle
{
    fn into_cs(self) -> String
    {
        u16::from(self).into_cs()
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

impl IntoCSCode for Value
{
    fn into_cs(self) -> String
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

impl IntoCSCode for Rarity
{
    fn into_cs(self) -> String
    {
        i8::from(self).into_cs()
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

impl IntoCSCode for DamageType
{
    fn into_cs(self) -> String
    {
        match self {
            DamageType::Melee => { "Item.DamageType = DamageClass.Melee;".to_string() }
            DamageType::Ranged => { "Item.DamageType = DamageClass.Ranged;".to_string() }
            DamageType::Magic => { "Item.DamageType = DamageClass.Magic;".to_string() }
            DamageType::Summon => { "Item.DamageType = DamageClass.Summon;".to_string() }
            DamageType::Thrown => { "Item.DamageType = DamageClass.Thrown;".to_string() }
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
    pub fn add_recipe(&mut self, ingredients: Vec<(ItemId, u16)>, stations: Vec<TileId>)
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
        todo!("Implement Item::set_projectile")
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
            use_sound: crate::terraria::sound_ids::items::Melee(),
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
    pub fn export(self, path: String, mod_backend_name: &str)
    {
        let mut fd : File = File::create({
            let mut s = path.to_string();
            s.push_str("/");
            s.push_str(&self.name);
            s.push_str(".cs");
            s
        }).expect("Could not create item file");
        fd.write(self.into_cs(mod_backend_name).as_bytes()).expect("Could not write to item file");
    }
}

impl Item
{
    pub fn into_cs(self, mod_name: &str) -> String
    {
        let class = crate::concat_cs_code!(
            "\
            using Terraria;\n\
            using Terraria.ID;\n\
            using Terraria.ModLoader;\n\
            \n\
            namespace ", mod_name, ".Items\n\
            {\n\
            \tpublic class ", self.name.clone(), " : ModItem \n\
            \t{\n\
            \t\tpublic override void SetStaticDefaults()\n\
            \t\t{\n\
            \t\t\t// DisplayName.SetDefault(\"", self.name.into_cs(),"\"); // By default, capitalization in classnames will add spaces to the display name. You can customize the display name here by uncommenting this line.\n\
            \t\t\tTooltip.SetDefault(\"", self.tooltip.into_cs(),"\");\n\
            \t\t}\n\
            \t\n\
            \t\tpublic override void SetDefaults()\n\
            \t\t{\n\
            ",//"\t\tItem.name = \"", &self.name.into_cs(), "\";\n",
            "\t\t\tItem.damage = ", &self.damage.into_cs(), ";\n\
            \t\t\tItem.knockBack = ", &self.knockback.into_cs(), ";\n\
            \t\t\t", &self.damage_type.expect("Optional damage type not supported yet").into_cs() ,"\
            \t\n\
            \t\t\tItem.consumable = ", &self.consumable.into_cs(), ";\n\
            \t\t\tItem.maxStack = ", &self.max_stack.into_cs(), ";\n\
            \t\n\
            \t\t\tItem.autoReuse = ", &self.auto_reuse.into_cs(), ";\n\
            \t\t\tItem.value = ", &self.value.into_cs(), ";\n\
            \t\t\tItem.rare = ", &self.rarity.into_cs(), ";\n\
            \t\n\
            \t\t\tItem.width = ", &self.width.into_cs(), ";\n\
            \t\t\tItem.height = ", &self.height.into_cs(), ";\n\
            \t\n\
            \t\t\tItem.useTime = ", &self.use_time.into_cs(), ";\n\
            \t\t\tItem.useAnimation = ", &self.use_animation.into_cs(), ";\n\
            \t\t\tItem.useStyle = ", &self.use_style.into_cs(), ";\n\
            \t\t\tItem.UseSound = ", &self.use_sound.into_cs(), ";\n\
            \t\t}\n\
            \n",
            {
                if self.recipes.len() != 0 {
                    let mut s = String::new();

                    s.push_str("\t\tpublic override void AddRecipes()\n\
                    \t\t{\n");

                    for recipe in self.recipes
                    {
                        s.push_str(recipe.into_cs(mod_name.to_string(), true).as_str());
                    }

                    s.push_str("\t\t}\n");

                    s
                }
                else { "".to_string() }
                .as_str()
            },
            "\t}\n\
            }"
        );

        class
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Recipe {
    result : ItemId,
    ingredients : Vec<(ItemId, u16)>,
    // groups: Vec<String>, // TODO : Integrate recipegroups (recipe.AddRecipeGroup(str)) (https://github.com/tModLoader/tModLoader/blob/master/ExampleMod/Items/ExampleItem.cs)
    stations : Vec<TileId>,
}

impl Recipe
{
    fn into_cs(self, mod_name: String, set_result_to_this: bool) -> String
    {
        let mut s = String::new();

        s.push_str("\t\t\tRecipe ");

        let recipe_name = { // Unique recipe name
            let mut h = DefaultHasher::new();
            self.stations.hash(&mut h);
            self.ingredients.hash(&mut h);
            let id = h.finish().to_string();

            let mut recipe_name = "recipe".to_string();
            recipe_name.push_str(&id);
            recipe_name
        };

        s.push_str(&recipe_name);

        s.push_str(" = CreateRecipe();\n");

        for (ingr, amount) in self.ingredients
        {
            s.push_str("\t\t\t");
            s.push_str(&recipe_name);
            s.push_str(".AddIngredient(");
            s.push_str(&ingr.into_cs());
            s.push_str(", ");
            s.push_str(&amount.to_string());
            s.push_str(");\n");
        }

        for tile in self.stations
        {
            s.push_str(&concat_cs_code!("\t\t\t", &recipe_name, ".AddTile("));
            s.push_str(&tile.into_cs(mod_name.to_string()));
            s.push_str(");\n");
        }

        s.push_str(&concat_cs_code!("\t\t\t", &recipe_name,
            if set_result_to_this
            {
                ".Register();\n".to_string()
            }
            else
            {
                panic!("set_result_to_this: false ; Recipe::to_cs(self, String, false) not impelment")
            }
        ));

        /*s.push_str("\t\t");
        s.push_str(&recipe_name.to_string());
        s.push_str(".AddRecipe();\n");*/

        s
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Projectile {
    id : ProjectileId,
    // TODO : Struct Projectile
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Tile {
    id : TileId,
    // TODO : Struct Tile
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Buff {
    id : BuffId,
    // TODO : Struct Buff
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Entity {
    id : EntityId,
    // TODO : Struct Entity
}
