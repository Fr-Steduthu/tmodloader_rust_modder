use crate::tmod_types::{Identifier, Item, Value, UseStyle, DamageType};
use crate::terraria_defaults::time::TICK;

impl Item {
    pub fn new(name : String, tooltip : String) -> Self {
        Item{
            id: Identifier { id: name.clone(), is_vanilla: false },
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

    pub fn usable(name : String, tooltip : String) -> Self {
        let mut item = Self::new(name, tooltip);

        item.use_time = 20 * TICK;
        item.use_animation = 20 * TICK;

        item.width = 20;
        item.height = 20;

        return item;
    }

    pub fn consumable(name : String, tooltip : String) -> Self {
        let mut item = Self::usable(name, tooltip);

        item.consumable = true;

        return item;
    }

    pub fn weapon(name : String, tooltip : String, damage : i64, damage_type : DamageType) -> Self {
        let mut item = Self::usable(name, tooltip);

        item.damage = damage;
        item.damage_type = Some(damage_type);

        return item;
    }
}

impl Item {
    pub fn sword(name: String, tooltip : String, damage : i64) -> Self {
        let mut item = Self::weapon(name, tooltip, damage, DamageType::Melee);

        item.use_turn = true;

        return item;
    }
}