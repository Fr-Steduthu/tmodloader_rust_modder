use crate::{
    CSCode,
    types::{
        CSClass,

        CSTypePrefix,
        CSType,
        CSPrimitive,

        AccessModifier,

        CSFunctionTerm,
        CSIntruction,
        CSLValue,
        CSRValue,

        CSMethod,
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
                    Into::<CSCode>::into(value.primitive_type()),
                    if value.is_array() { "[]" } else { "" },
            )
        )
    }
}

impl From<CSPrimitive> for CSCode {
    fn from(value: CSPrimitive) -> Self {
        CSCode::from(
            format!("{}", match value {
                CSPrimitive::String => "string".to_string(),
                CSPrimitive::Integer => "integer".to_string(),
                CSPrimitive::Float => "float".to_string(),
                CSPrimitive::Void => "void".to_string(),
                CSPrimitive::Class(n) => n,
            })
        )
    }
}

impl TryFrom<CSRValue> for CSCode {
    fn try_from(value : CSRValue) -> Result<Self, Self::Error> {
        match value {
            CSRValue::Litteral(n, _) => CSCode::from(n),
            CSRValue::LValue(lv) => Ok(lv.into()),
            CSRValue::FuncCall(term, args) => {
                Ok(CSCode::from(
                    match term {
                        CSFunctionTerm::Function(csmethod) =>
                            format!(
                                "{}({})",
                                csmethod.name(),
                                {
                                    let mut v = vec![];
                                    for b in args.iter() {
                                        v.push(*b.to_string());
                                    }
                                    v.join(", ")
                                }
                            ),
                        CSFunctionTerm::ExternalFunction(name, input, output) => {
                            format!("{name}({})",
                                {
                                    let mut v = vec![];
                                    for b in args.iter() {
                                        v.push(*b.to_string());
                                    }
                                    v.join(", ")
                                }
                            )
                        }
                    }
                ))
            }
        }
    }
}

impl From<CSLValue> for CSCode {
    fn from(value : CSLValue) -> CSCode {
        CSCode::from(value.identifier())
    }
}

impl From<CSFunctionTerm> for CSCode {
    fn from(value : CSFunctionTerm) -> Self {
        CSCode::from(
            match value {
                CSFunctionTerm::Function(f) => f.name(),
                CSFunctionTerm::ExternalFunction(f, _, _) => f.to_string()
            }
        )
    }
}

impl From<CSIntruction> for CSCode {
    fn from(value : CSIntruction) -> Self {
        CSCode::from(
            format!("{}",
                    todo!("From<CSCode> for CSIntruction")
            )
        )
    }
}

impl From<CSClass> for CSCode {
    fn from(value : CSClass) -> CSCode {
        todo!("Ajouter les imports de namespace");
        todo!("From<CSClass> for CSCode");
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

impl From<CSMethod> for CSCode {
    fn from(value : CSMethod) -> CSCode {
        todo!("From<CSCode> for CSMethod")
    }
}