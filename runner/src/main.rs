use library::cs::CSTemplate;
use library::tmod_types::{Item, Mod};

fn main() {
    println!("Hi, I'm the development project!");

    let it = Item::sword("Toto".to_string(), "Tata".to_string(), 64);

    println!("{}", it.to_cs())
}

#[test]
fn test_mod_export()
{
    let m = Mod::init(
        "TestMod",
        "GeneratedMod",
        "Steduthu",
        "0.1",
        "Andu'fallah'dor"
    );

    m.export("./mymod");
}
