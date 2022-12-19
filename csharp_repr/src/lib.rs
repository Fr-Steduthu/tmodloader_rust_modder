#[cfg(test)] use crate::project::{CSNamespace, CSProject};
#[cfg(test)] use crate::types::CSClass;

pub mod types;
pub mod project;

pub trait ToCS {
    fn to_cs(self) -> String;
}

pub trait IO {
    fn to_string(&self) -> String;
    fn from_string(data : &String) -> Self where Self : Sized;
}

#[test]
fn main() {
    let mut proj : CSProject = CSProject::new("Terraria mod project test 1");
    let namespace : &mut CSNamespace = proj.new_namespace("crate::root::package");
    let mod_item : &mut CSClass = namespace.new_class("ModItem");
}

#[test]
fn test_nameplaces_names() {
    let reg = regex::Regex::new(r"(?:@?[a-zA-Z]\w*(?:\.@?[a-zA-Z_]\w*)*)").unwrap();

    if reg.is_match("1") { panic!("panicked at 1") }
    if reg.is_match(".") { panic!("panicked at .") }
    if reg.is_match("_") { panic!("panicked at _") }
    if reg.is_match("a.1") { panic!("panicked at a.1") }
    if reg.is_match("a1.1") { panic!("panicked at a1.1") }

    if !reg.is_match("a.a") { panic!("panicked at a.a") }
    if !reg.is_match("a") { panic!("panicked at a") }
    if !reg.is_match("a1") { panic!("panicked at a1") }
    if !reg.is_match("a.a_1") { panic!("panicked at a.a_1") }
    if !reg.is_match("a.a1_") { panic!("panicked at a.a1_") }
    if !reg.is_match("a_") { panic!("panicked at a_") }
    if !reg.is_match("a_.a") { panic!("panicked at a_.a") }
}