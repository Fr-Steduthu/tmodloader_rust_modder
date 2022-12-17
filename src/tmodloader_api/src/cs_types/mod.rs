#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CSType {
    prefix : CSTypePrefix,
    t : CSPrimalType,
    is_array: bool,
}

impl From<CSPrimalType> for CSType {
    fn from(value: CSPrimalType) -> Self {
        CSType{
            prefix: CSTypePrefix::None,
            t: value,
            is_array: false,
        }
    }
}

impl ToString for CSType {
    fn to_string(&self) -> String {
        [
            self.prefix.to_string().as_str(),
            self.t.to_string().as_str(),
            if self.is_array { "[]" } else { "" }
        ].join(" ")
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
enum CSTypePrefix{ None, Ref, Out, Params}

impl ToString for CSTypePrefix {
    fn to_string(&self) -> String {
        match self {
            CSTypePrefix::Ref => { "ref".to_string() }
            CSTypePrefix::Out => { "out".to_string() }
            CSTypePrefix::Params => { "params".to_string() }
            CSTypePrefix::None => { "".to_string() }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum CSPrimalType {
    String,
    Integer,
    Float,
    Void,
    Custom(String)
}

impl ToString for CSPrimalType {
    fn to_string(&self) -> String {
        match self {
            CSPrimalType::String => { "string".to_string() }
            CSPrimalType::Integer => { "int".to_string() }
            CSPrimalType::Float => { "float".to_string() }
            CSPrimalType::Custom(name) => { name.clone() }
            CSPrimalType::Void => { "void".to_string() }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CSFunction {
    name : String,
    access : AccessModifier,
    arguments : Vec<(String, CSType)>,

    body : String,

    return_value : CSType,

    is_override : bool,
    scoped_variables : Vec<(String, CSType)>
}

impl ToString for CSFunction {
    fn to_string(&self) -> String {
        {
            let mut ret_val = vec![
                if self.is_override { "override" } else { "" }.to_string(), self.access.to_string(), self.return_value.to_string(), self.name.clone()
            ];

            for (name, t) in &self.arguments {
                ret_val.push(t.to_string());
                ret_val.push(name.clone());
            }

            ret_val.append(&mut vec![
                "{\n".to_string(),
                self.body.clone(), "\n".to_string(),
                "}".to_string()
            ]);
            ret_val
        }.join(" ").to_string()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum AccessModifier {
    Private,
    Protected,
    Public,
}
impl ToString for AccessModifier {
    fn to_string(&self) -> String {
        match self {
            AccessModifier::Private => { "private".to_string() }
            AccessModifier::Protected => { "protected".to_string() }
            AccessModifier::Public => { "public".to_string() }
        }
    }
}
