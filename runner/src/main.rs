use library::cs::IntoCSCode;
use library::tmod_types::{DamageType, Item, ItemId, Mod};
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

            it.add_recipe(
                vec![
                    (library::terraria::item_ids::DirtBlock(), 1),
                ],
                vec![
                    library::terraria::tile_ids::DirtTile(),
                ]
            );

            it
        }
    );

    m.export(r"C:\Users\erwan\Documents\My Games\Terraria\tModLoader\ModSources\");
    println!("Project has been exported");
}

