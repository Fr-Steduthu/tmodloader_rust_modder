
fn main() {
    println!("Hello, world!");

    let item = tmodloader_api::tmod_types::Item::new("tyuhvb".to_string(), "tyughuj".to_string());
    println!("{}", serde_json::to_string(&item.clone()).unwrap());
    println!("{}", item.to_string());
}
