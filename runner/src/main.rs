use library::cs::IntoCSCode;
use library::terraria::time::irl::seconds;
use library::tmod_types::{DamageType, Item, ItemId, Mod, Time, UseStyle};
use library::tmod_types::Identifier::Terraria;

fn main() {
    println!("Hi, I'm the development project!");

    let mut m = Mod::init(
        "TestMod",
        "Generated mod",
        "Steduthu",
        "0.1",
        "Andu'falah'dor"
    );

    m.add_item(
        Item::sword(
            "Toto".to_string(),
            "Tata".to_string(),
            64
        )
    );
    m.add_item(
        {
            let mut it = Item::weapon(
                "DirtblockWeapon".to_string(),
                "Some summoning damage test weapon".to_string(),
                1,
                DamageType::Summon
            );

            it.set_style(
                seconds(1),
                seconds(1),
                UseStyle::HoldOut,
                library::terraria::sound_ids::items::Melee(),
                false,
                false,
                true,
                true,
            );

            it.add_recipe(
                vec![
                    (library::terraria::item_ids::DirtBlock(), 1),
                ],
                vec![
                    library::terraria::tile_ids::Workbenches(),
                ]
            );

            it
        }
    );

    m.export(r"C:\Users\erwan\Documents\My Games\Terraria\tModLoader\ModSources\");
    println!("Project has been exported");
}

