use csharp_repr::ToCS;
use super::{
    Identifier,
    DamageType,
    CSType,
    CSClass,
    CSFunction,
    Item, Value,
    Recipe,
    Projectile, Tile, Buff, Entity
};

impl ToCS for Identifier {
    fn to_cs(self) -> String {
        match self {
            Identifier::Terraria(id) => { id.to_string() }
            Identifier::Modded(id) => { id.clone() }
        }
    }
}

impl Into<CSType> for Identifier {
    fn into(self) -> CSType {
        todo!()
    }
}

impl ToCS for DamageType {
    fn to_cs(self) -> String {
        match self {
            DamageType::Melee => { "melee".to_string() }
            DamageType::Ranged => { "ranged".to_string() }
            DamageType::Magic => { "magic".to_string() }
            DamageType::Summon => { "summon".to_string() }
            DamageType::Thrown => { "thorwn".to_string() }
        }
    }
}

impl Into<CSClass> for Item {
    fn into(self) -> CSClass {
        CSClass {
            name: self.name.clone(),
            namespace: "ModItem".to_string(),
            accessibility: super::AccessModifier::Public,
            parent_classes: vec!["ModItem".to_string()],
            fields: vec![],
            functions: vec![
                CSFunction::new("setDefaults".to_string(), vec![], CSType::void(),
                                vec![
                                    "\t\titem.name = ".to_string(), self.name, ";\n".to_string(),
                                    "\t\titem.tooltip = \"".to_string(), self.tooltip, "\";\n".to_string(),
                                    if <Value as Into<u64>>::into(self.value.clone()) != 0u64 { ["\t\titem.value = ", <Value as Into<u64>>::into(self.value.clone()).to_string().as_str(), ";\n"].join("") } else { "".to_string() },
                                    if let Some(t) = self.damage_type { ["\t\titem.", t.to_cs().as_str(), " = true;\n\t\titem.damage = ", self.damage.to_string().as_str(), "\n"].join("") } else { "".to_string() },
                                    if self.use_turn { "\t\tthis.useTurn = true;\n" } else { "" }.to_string(),
                                    if self.auto_reuse { "\t\tthis.autoReuse = true;\n"} else { "" }.to_string(),
                                ]
                )
            ],
        }
    }
}

impl Into<CSClass> for Recipe {
    fn into(self) -> CSClass {
        todo!()
    }
}

impl Into<CSClass> for Projectile {
    fn into(self) -> CSClass {
        todo!()
    }
}

impl Into<CSClass> for Tile {
    fn into(self) -> CSClass {
        todo!()
    }
}

impl Into<CSClass> for Buff {
    fn into(self) -> CSClass {
        todo!()
    }
}

impl Into<CSClass> for Entity {
    fn into(self) -> CSClass {
        todo!()
    }
}
