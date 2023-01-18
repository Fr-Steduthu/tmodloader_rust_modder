use std::thread::AccessError;
use crate::{
    CSCode,
    types::{
        CSClass,
        CSTypePrefix,
        CSType,
        _CSType,
        AccessModifier,
        CSFunctionTerm,
        CSIntruction,
        CSLValue,
        CSRValue,
    }
};

impl From<CSTypePrefix> for CSCode {
    fn from(value: CSTypePrefix) -> Self {
        CSCode::from(
            format!("{}", match value {
                CSTypePrefix::Ref => { "ref" }
                CSTypePrefix::Out => { "out" }
                CSTypePrefix::Params => { "params" }
            })
        )
    }
}

impl From<CSType> for CSCode {
    fn from(value: CSType) -> Self {
        CSCode::from(
            format!("{} {} {}",
                    if let Some(pref) = &value.prefix() {
                        Into::<CSCode>::into(pref.clone()).to_string()
                    } else {
                        "".to_string()
                    },
                    Into::<CSCode>::into(value.cstype()),
                    if value.is_array() { "[]" } else { "" },
            )
        )
    }
}

impl From<_CStype> for CSCode {
    fn from(value: _CStype) -> Self {
        CSCode::from(
            format!("{}", match value {
                _CSType::String => "string".to_string(),
                _CSType::Integer => "integer".to_string(),
                _CSType::Float => "float".to_string(),
                _CSType::Void => "void".to_string(),
                _CSType::Class(n) => n,
            })
        )
    }
}

impl From<CSRValue> for CSCode {
    fn from(value : CSRValue) -> Self {
        match value {
            CSRValue::Litteral(n, _) => CSCode::from(n),
            CSRValue::LValue(lv) => lv.into(),
        }
    }
}

impl From<CSLValue> for CSCode {
    fn from(value : CSLValue) -> CSCode {
        CSCode::from(value.identifier())
    }
}

impl From<CSFunctionTerm> for CSCode {
    fn from(value : CSFunctionTerm) -> CSCode {
        CSCode::from(
            match value {
                CSFunctionTerm::Function(f) => f.name(),
                CSFunctionTerm::ExternalFunction(f, _, _) => f.to_string()
            }
        )
    }
}


impl From<CSIntruction> for CSCode {
    fn From(value : CSIntruction) -> CSCode {
        CSCode::from(
            format!("{}",
                    todo!("From<CSCode> for CSIntruction")
            )
        )
    }
}

impl From<CSClass> for CSCode {
    fn From(value : CSClass) -> CSCode {
        todo!("From<CSClass> for CSCode")
    }
}

impl From<AccessModifier> for CSCode {
    fn from(value : AccessModifier) -> CSCode {
        CSCode::from(
            match value {
                AccessModifier::Private => "private ",
                AccessModifier::Protected => "protected ",
                AccessModifier::Public => "public ",
            }.to_string()
        )
    }
}

impl From<CSCode> for CSMethod {
    fn From(self) -> CSCode {
        todo!("From<CSCode> for CSMethod")
    }
}