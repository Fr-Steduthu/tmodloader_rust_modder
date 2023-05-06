use crate::cs::CSTemplate;
use crate::tmod_types::{Identifier, Item, Value, UseStyle, DamageType};
use crate::terraria_defaults::time::TICK;

impl Item {
    pub fn new(name : String, tooltip : String) -> Self
    {
        Item{
            id: Identifier::Modded(name.clone()),
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
}

impl Item {
    pub fn sword(name: String, tooltip : String, damage : i64) -> Self
    {
        let mut item = Self::weapon(name, tooltip, damage, DamageType::Melee);

        item.use_turn = true;

        return item;
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
            "
        );

        class.push_str("}");
        class

        /*

        // Debug recipe // TODO : Remove
        "\tpublic override void AddRecipes()",
        "\t{",
            "\t\tModRecipe recipe = new ModRecipe(mod);",
            //"\t\trecipe.AddIngredient(ItemID.DirtBlock, 10);",
            "\t\trecipe.AddTile(TileID.WorkBenches);",
            "\t\trecipe.SetResult(this);",
            "\t\trecipe.AddRecipe();",
        "\t}",
        */
    }
}