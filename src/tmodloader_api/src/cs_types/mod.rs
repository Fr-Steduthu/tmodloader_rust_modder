#[derive(serde::Serialize, serde::Deserialize)]
pub enum CSType {
    String,
    Integer,
    Float,
    Custom(String)
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CSObject {
    classname : String,
    namespace : String,
    accessibility : AccessModifier,

    fields : Vec<(String, CSType)>,
    functions : Vec<CSFunction>,
}

impl ToString for CSObject {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CSFunction {
    name : String,
    access : AccessModifier,
    arguments : Vec<(String, CSType)>,

    body : String,

    return_value : CSType,

    is_override : bool,
    scoped_variables : Vec<(String, CSType)>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AccessModifier {
    Private,
    Protected,
    Public,
}
