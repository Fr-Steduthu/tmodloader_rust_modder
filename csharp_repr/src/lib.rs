#[cfg(test)] use crate::project::{CSNamespace, CSProject};
#[cfg(test)] use crate::types::{CSClass};
use crate::types::{CSIntruction, CSType};

pub mod types;
pub mod project;

//#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSCode {
    content : String,
}

impl From<String> for CSCode {
    fn from(value: String) -> Self {
        CSCode{ content: value.clone() }
    }
}

impl std::fmt::Display for CSCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.clone())
    }
}

pub mod into_cscode;
pub mod serialization;

#[test]
fn main() {
    let mut proj : CSProject = CSProject::new("Terraria mod project test 1");
    #[allow(non_snake_case)] let MyItemsNamespace = { proj.new_namespace("MyItems"); proj.namespace_mut("MyItems").unwrap() };
    #[allow(non_snake_case)] let MyItems_General_Namespace = proj.get_new_namespace("MyItems.General");
}

#[test]
fn test_nameplaces_names() {
    let reg = crate::project::NAMESPACE_NAME_REGEX.clone();

    if reg.is_match("1") { panic!("panicked at 1") }
    if reg.is_match(".") { panic!("panicked at .") }
    if reg.is_match("_") { panic!("panicked at _") }
    if reg.is_match("a.1") { panic!("panicked at a.1") }
    if reg.is_match("a1.1") { panic!("panicked at a1.1") }

    if !reg.is_match("a.a") { panic!("panicked at a.a") }
    if !reg.is_match("a_.a") { panic!("panicked at a_.a") }
    if !reg.is_match("a_.@a") { panic!("panicked at a_.a") }
    if !reg.is_match("a.a_1") { panic!("panicked at a.a_1") }
    if !reg.is_match("a.a1_") { panic!("panicked at a.a1_") }
    if !reg.is_match("a") { panic!("panicked at a") }
    if !reg.is_match("a1") { panic!("panicked at a1") }
    if !reg.is_match("a_") { panic!("panicked at a_") }
}

#[test]
fn test_CSCode_instruction() {
    todo!("Implement test")
}

#[test]
fn test_inference() {
    let a = CSIntruction::declare("a", CSType::integer(), CSIntruction::litteral("3"));
}