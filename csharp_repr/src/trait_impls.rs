use crate::ToCS;
use super::{CSClass, CSType, _CSType, CSTypePrefix, AccessModifier};

impl ToCS for CSType {
    fn to_cs(self) -> String {
        [
            if let Some(pref) = &self.prefix { pref.clone().to_cs() } else { "".to_string() }.as_str(),
            self.t.to_cs().as_str(),
            if self.is_array { "[]" } else { "" }
        ].join(" ")
    }
}

impl ToCS for CSTypePrefix {
    fn to_cs(self) -> String {
        match self {
            CSTypePrefix::Ref => { "ref".to_string() }
            CSTypePrefix::Out => { "out".to_string() }
            CSTypePrefix::Params => { "params".to_string() }
        }
    }
}

impl ToCS for _CSType {
    fn to_cs(self) -> String {
        match self {
            _CSType::String => { "string".to_string() }
            _CSType::Integer => { "int".to_string() }
            _CSType::Float => { "float".to_string() }
            _CSType::Class(name) => { name.clone() }
            _CSType::Void => { "void".to_string() }
        }
    }
}

impl ToCS for CSClass {
    fn to_cs(self) -> String {
        todo!();
        "Not implemented yet".to_string()
    }
}


impl ToCS for AccessModifier {
    fn to_cs(self) -> String {
        match self {
            AccessModifier::Private => { "private".to_string() }
            AccessModifier::Protected => { "protected".to_string() }
            AccessModifier::Public => { "public".to_string() }
        }
    }
}
