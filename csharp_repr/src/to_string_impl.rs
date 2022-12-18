use super::{CSClass, CSType, CSPrimalType, CSTypePrefix, AccessModifier};

impl ToString for CSType {
    fn to_string(&self) -> String {
        [
            if let Some(pref) = &self.prefix { pref.to_string() } else { "".to_string() }.as_str(),
            self.t.to_string().as_str(),
            if self.is_array { "[]" } else { "" }
        ].join(" ")
    }
}

impl ToString for CSTypePrefix {
    fn to_string(&self) -> String {
        match self {
            CSTypePrefix::Ref => { "ref".to_string() }
            CSTypePrefix::Out => { "out".to_string() }
            CSTypePrefix::Params => { "params".to_string() }
        }
    }
}

impl ToString for CSPrimalType {
    fn to_string(&self) -> String {
        match self {
            CSPrimalType::String => { "string".to_string() }
            CSPrimalType::Integer => { "int".to_string() }
            CSPrimalType::Float => { "float".to_string() }
            CSPrimalType::Custom(name) |
            CSPrimalType::Class(name) => { name.clone() }
            CSPrimalType::Void => { "void".to_string() }
        }
    }
}

impl ToString for CSClass {
    fn to_string(&self) -> String {
        todo!();
        "Not implemented yet".to_string()
    }
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
