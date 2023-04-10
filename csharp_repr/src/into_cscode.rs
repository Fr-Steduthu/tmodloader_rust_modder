use std::ops::Deref;
use AsRef;
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

macro_rules! CSFormat {
    ($str:literal $(,$arg:expr)*) => {
        format!($str $(,$crate::CSCode::try_from($arg).unwrap().to_string())*)
    }
}

impl<T : Into<CSCode>> From<Box<T>> for CSCode {
    fn from(value: Box<T>) -> Self {
        (*value).into()
    }
}
impl<T : TryInto<CSCode> + Clone> TryFrom<Vec<T>> for CSCode {
    type Error = String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let mut v = vec![];
        for b in value.iter() {
            v.push(match <T as TryInto<CSCode>>::try_into(b.clone()) {
                Ok(v) => v,
                Err(_) => return Err(format!("Could not convert {} to CSCode", std::any::type_name::<T>())),
            }.to_string());
        }
        Ok(v.join(", ").into())
    }
}

impl TryFrom<Vec<Box<CSRValue>>> for CSCode {
    type Error = String;
    fn try_from(value: Vec<Box<CSRValue>>) -> Result<Self, Self::Error> {
        let mut v = vec!["(".to_string()];
        for bx in value {
            v.push(CSCode::from(*bx.clone()).to_string())
        }
        v = v.join(", ");
        v.push(")".to_string());

        CSCode::from(v)
    }
}

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

// Values

impl TryFrom<CSRValue> for CSCode {
    type Error = String;

    fn try_from(value : CSRValue) -> Result<Self, Self::Error> {
        match value {
            CSRValue::Litteral(n, _) => Ok(CSCode::from(n)),
            CSRValue::LValue(lv) => Ok(lv.into()),
            CSRValue::FuncCall(term, args) => {
                Ok(CSCode::from(
                    match term {
                        CSFunctionTerm::Function(csmethod) => format!("{}({})", csmethod.name(), CSFormat!("{}", args)),
                        CSFunctionTerm::ExternalFunction(name, input, _) => format!("{name}({})", CSCode::try_from(input).unwrap()),
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
                CSFunctionTerm::ExternalFunction(f, _, _) => f
            }
        )
    }
}

impl From<CSIntruction> for CSCode {
    fn from(value : CSIntruction) -> Self {
        CSCode::from(
            format!("{}",
                    match value {
                        CSIntruction::Declaration(id, ty, val) => CSFormat!("{} {} = {};", ty, id, val),
                        CSIntruction::Affect(id, rv) => CSFormat!("{} = {};", id, rv),
                        CSIntruction::AffectToCall(id, func, args) => CSFormat!("{} = {}({})", id, func, args),
                        CSIntruction::Call(fun, args) => CSFormat!("{}({})", fun, args),
                        CSIntruction::Return(ret) => CSFormat!("{}", ret),
                    }
            )
        )
    }
}

// Class & functions

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

// Type inference

pub trait Inferable<Return> {
    type Error;
    fn infer(&self) -> Result<Return, Self::Error>;
}

impl Inferable<CSType> for CSLValue {
    type Error = String;
    fn infer(&self) -> Result<CSType, Self::Error> {
        Ok(self.cstype())
    }
}
impl Inferable<CSType> for CSRValue {
    type Error = String;
    fn infer(&self) -> Result<CSType, Self::Error> {
        match self {
            CSRValue::Litteral(id, ty) => Ok(ty.clone()),
            CSRValue::LValue(lv) => Ok(lv.infer()?),
            CSRValue::FuncCall(fun, args) => {
                fun.infer(&args.into_iter().map(move |o| {*o.clone()} ).collect::<Vec<CSRValue>>())?;
                Ok(fun.return_type())
            }
        }
    }
}
impl CSFunctionTerm {
    pub(crate) fn return_type(&self) -> CSType {
        match self {
            CSFunctionTerm::Function(cs) => cs.return_type(),
            CSFunctionTerm::ExternalFunction(_, _, out) => out.clone(),
        }
    }

    pub(crate) fn infer(&self, v : &Vec<CSRValue>) -> Result<CSType, String> {
        Ok(match self {
            CSFunctionTerm::Function(csmethod) => {
                for ((_, a), b) in csmethod.arguments().iter().zip(v.iter()) {
                    if a.clone() != b.infer()? {
                        return Err("Incompatible types in method call".to_string())
                    }
                }
                csmethod.return_type()
            }
            CSFunctionTerm::ExternalFunction(_, input, output) => {
                for (a, b) in v.iter().zip(input.iter()) {
                    if a.infer()? != b.clone() {
                        return Err("Incompatible types in method call".to_string())
                    }
                }
                output.clone()
            }
        })
    }
}
impl Inferable<CSType> for CSIntruction {
    type Error = String;
    fn infer(&self) -> Result<CSType, Self::Error> {
        match self {
            CSIntruction::Declaration(id, ty, expr) => {
                let ty = ty.clone();
                if ty == expr.infer()? {
                    return Ok(ty)
                }
                Err("Incompatible types at declaration".to_string())
            }
            CSIntruction::Affect(id, expr) => {
                let ty = id.infer()?;
                if ty == expr.infer()? {
                    return Ok(ty)
                }
                Err("Incompatible types in affectation".to_string())
            }
            CSIntruction::AffectToCall(id, func, args) => {
                let idty = id.infer()?;
                if func.infer(args)? == id.infer()? {
                    return Ok(idty)
                }
                Err("Incompatible types".to_string())
            }
            CSIntruction::Call(functerm, args) => {
                functerm.infer(args)?;
                return Ok(CSType::void())
            }
            CSIntruction::Return(v) => Ok(v.infer()?)
        }
    }
}