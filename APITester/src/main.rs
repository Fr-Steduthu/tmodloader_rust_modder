#![allow(non_snake_case)]

use tmodloader_api::{
    csharp_repr::{
        types::CSClass,
        ToCS,
    },
    tmod_types::{
        Item,
    }
};

#[cfg(test)]
macro_rules! startup {
    ( $item : ident ) => {
        let $item : Item = Item::new("Dummy item".to_string(), "Item created for testing purposes".to_string() );
    };
}

fn main() {
    println!("Hello, world!");

    let item = Item::new("tyuhvb".to_string(), "tyughuj".to_string());
    println!("{}", serde_json::to_string(&item.clone()).unwrap());
    println!("{}", <Item as Into<CSClass>>::into(item).to_cs());
}



#[test]
fn test_item_to_csobj(){
    startup!(item);
    println!("{:?}", <Item as Into<CSClass>>::into(item));
}