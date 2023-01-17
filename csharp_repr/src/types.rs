use std::fmt::{Display, Formatter};

use crate::project::{CSNamespace, CSCode, CSFile};

/**********************************
*             CSType              *
**********************************/

#[derive(Clone, Debug, PartialEq)]
pub struct CSType {
    prefix : Option<CSTypePrefix>,
    t : _CSType,
    is_array: bool,
}

impl CSType {
    pub fn void() -> Self {
        CSType {
            prefix: None,
            t: _CSType::Void,
            is_array: false,
        }
    }

    pub fn integer() -> Self {
        CSType {
            prefix: None,
            t: _CSType::Integer,
            is_array: false,
        }
    }

    pub fn string() -> Self {
        CSType {
            prefix : None,
            t : _CSType::String,
            is_array : false,
        }
    }

    pub fn class(name : String) -> Self {
        CSType {
            prefix: None,
            t: _CSType::Class(name),
            is_array: false,
        }
    }
}

impl Display for CSType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}",
               match &self.prefix {
                   None => { "".to_string() }
                   Some(v) => { [v.clone().to_string().as_str(), " "].join("") }
               },
               self.t.to_string(),
               match self.is_array {
                   true => { "[]" }
                   false => { "" }
               }
        )
    }
}

impl Into<CSCode> for CSType {
    fn into(self) -> CSCode {
        CSCode::from(
            [
                if let Some(pref) = &self.prefix { pref.clone().to_cs() } else { "".to_string() }.as_str(),
                self.t.to_cs().as_str(),
                if self.is_array { "[]" } else { "" }
            ].join(" ")
        )
    }
}

#[allow(unused)]
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CSTypePrefix{
    Ref,
    Out,
    Params
}

impl Display for CSTypePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CSTypePrefix::Ref => { "ref" }
            CSTypePrefix::Out => { "out" }
            CSTypePrefix::Params => { "params" }
        })
    }
}

impl Into<CSCode> for CSTypePrefix {
    fn into(self) -> CSCode {
        format!("{}", match self {
            CSTypePrefix::Ref => { "ref" }
            CSTypePrefix::Out => { "out" }
            CSTypePrefix::Params => { "params" }
        })
    }
}

#[allow(unused)]
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum _CSType {
    String,
    Integer,
    Float,
    Void,
    Class(String),
}

impl Display for _CSType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            _CSType::String => { "string" }
            _CSType::Integer => { "integer"}
            _CSType::Float => { "float" }
            _CSType::Void => { "void" }
            _CSType::Class(n) => { n }
        })
    }
}

impl Into<CSCode> for _CSType {
    fn into(self) -> CSCode {
        format!("{}", match self {
            _CSType::String => "string",
            _CSType::Integer => "integer",
            _CSType::Float => "float",
            _CSType::Void => "void",
            _CSType::Class(n) => n,
        })
    }
}

/**********************************
*             CSValue             *
**********************************/

#[derive(Clone, Debug)]
pub enum CSValue {
    Variable(String, CSType),
    Litteral(String, CSType),
    Function(CSFunction),
    ExternalFunction(String, Vec<CSType>, CSType),
}

#[allow(unused)]
impl CSValue {
    fn identifier(&self) -> String {
        match self {
            CSValue::Variable(n, _) => { n.clone() }
            CSValue::Litteral(n, _) => { n.clone() }
            CSValue::Function(f) => { f.name.clone() }
            CSValue::ExternalFunction(n, _, _) => { n.clone() }
        }
    }
    
    fn cstype(&self) -> CSType {
        match self {
            CSValue::Variable(_, t) |
            CSValue::Litteral(_, t) => { t.clone() }
            CSValue::Function(csfunc) => { csfunc.return_value.clone() }
            CSValue::ExternalFunction(_, _, ret) => { ret.clone() }
        }
    }

    fn is_rvalue(&self) -> bool {
        match self {
            CSValue::Variable(_, _) |
            CSValue::Litteral(_, _) => true,
            _ => false
        }
    }

    fn is_function(&self) -> bool {
        match self {
            CSValue::Function(_) |
            CSValue::ExternalFunction(_, _, _) => true,
            _ => false
        }
    }

    fn arguments(&self) -> Result<Vec<CSType>, String> {
        match self {
            CSValue::Function(f) => {
                let mut ret = vec![];
                for (_, t) in &f.arguments {
                    ret.push(t.clone());
                }
                Ok(ret)
            }
            CSValue::ExternalFunction(_, args, _) => { Ok(args.clone()) }
            _ => { Err("Not a function".to_string()) }
        }
    }
}

impl Display for CSValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CSValue::Variable(n, _) |
            CSValue::Litteral(n, _)  => { write!(f, "{n}") }
            CSValue::Function(func) => { write!(f, "{func}") }
            CSValue::ExternalFunction(name, args, ret) => {
                write!(f, "{}({} -> {})",
                       name,
                       {
                           let mut s : Vec<String> = vec!["".to_string()];
                           for t in args {
                               s.push(t.to_string());
                           }
                           s.join(", ")
                       },
                       ret
                )
            }
        }
    }
}

impl Into<CSCode> for CSValue {
    fn into(self) -> CSCode {
        CSCode::from(
            match self {
                CSValue::Variable(n, _) |
                CSValue::Litteral(n, _)  => { format!("{n}") }
                CSValue::Function(func) => { format!("{func}") }
                CSValue::ExternalFunction(name, args, ret) => {
                    format!("{name}({} -> {})",
                        {
                           let mut s : Vec<String> = vec!["".to_string()];
                           for t in args {
                               s.push(t.into::<CSCode>().to_string());
                           }
                           s.join(", ")
                        },
                        ret.into::<CSCode>()
                    )
                }
            }
        )
    }
}

/**********************************
*          CSInstruction          *
**********************************/

#[derive(Clone, Debug)]
pub enum CSIntruction {
    Value(CSValue),
    Call(Box<CSValue>, Vec<CSValue>),
    Affect(Box<CSValue>, Box<CSValue>),
}

impl Display for CSIntruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CSIntruction::Call(fun, args) => {
                format!(
                    "{}({})",
                    match &fun {
                        &CSValue::Function(csobj) => { csobj.name() }
                        &CSValue::ExternalFunction(name, _, _) => { name }
                        _ => return Err("Cannot call a value as you would a function")
                    },
                    {
                        //TODO : Verifier le matching des types
                        let mut v = vec![];
                        for arg in args {
                            v.push(arg.to_string())
                        }
                        v.join(", ")
                    }
                )
            }
            CSIntruction::Affect(lvalue, rvalue) => format!("{lvalue} = {rvalue};"),
            CSIntruction::Value(v) => format!("return {v};"),
        })
    }
}

impl Into<CSCode> for CSIntruction {
    fn into(self) -> CSCode {
        CSCode::from(
            format!("{}",
                    match self {
                        CSIntruction::Call(fun, args) => {
                            format!(
                                "{}({})",
                                match &fun {
                                    &CSValue::Function(csobj) => { csobj.name() }
                                    &CSValue::ExternalFunction(name, _, _) => { name }
                                    _ => return Err("Cannot call a value as you would a function")
                                },
                                {
                                    //TODO : Verifier le matching des types
                                    let mut v = vec![];
                                    for arg in args {
                                        v.push(arg.into::<CSCode>().to_string())
                                    }
                                    v.join(", ")
                                }
                            )
                        }
                        CSIntruction::Affect(lvalue, rvalue) => format!("{lvalue} = {rvalue};"),
                        CSIntruction::Value(v) => format!("return {};", v.into::<CSCode>()),
                    }
            )
        )
    }
}

#[allow(unused)]
impl CSIntruction {
    pub(crate) fn get_type(self) -> Result<CSType, String> {
        match self {
            CSIntruction::Call(func, arg_values) => {
                if !func.is_function() {
                    return Err(format!("{} is not a function", func.identifier()));
                }

                for (arg_v, arg_t) in arg_values.iter().zip(func.arguments().unwrap()) {
                    if arg_v.cstype() != arg_t {
                        return Err(format!("Type missmatch : {} != {}", arg_v.cstype(), arg_t))
                    }
                }

                Ok(func.cstype())
            }
            CSIntruction::Affect(rvalue, lvalue) => {
                if !rvalue.is_rvalue() {
                    return Err(format!("{rvalue} is not a rvalue"));
                }

                if rvalue.cstype() != lvalue.cstype() {
                    return Err(format!("Type mismatch : {} != {}", rvalue.cstype(), lvalue.cstype()));
                }

                Ok(CSType::void())
            }
            CSIntruction::Value(csvalue) => { Ok(csvalue.cstype()) }
        }
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum AccessModifier {
    Private,
    Protected,
    Public,
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AccessModifier::Private => { "private " }
            AccessModifier::Protected => { "protected " }
            AccessModifier::Public => { "public " }
        })
    }
}

impl Into<CSCode> for AccessModifier {
    fn into(self) -> CSCode {
        CSCode::from(
            match self {
                AccessModifier::Private => { "private " }
                AccessModifier::Protected => { "protected " }
                AccessModifier::Public => { "public " }
            }
        )
    }
}

/**********************************
*             CSCLASS             *
**********************************/

#[derive(Clone, Debug)]
pub struct CSClass {
    pub name : String,
    pub namespace : * const CSNamespace,
    pub accessibility : AccessModifier,

    pub parent_classes : Vec<String>,

    pub fields : Vec<(String, CSType)>,
    pub functions : Vec<CSFunction>,
}

impl CSClass {
    pub(crate) fn new(name : &str, namespace : *const CSNamespace) -> Self {
        CSClass {
            name : name.to_string(),
            namespace,
            accessibility: AccessModifier::Private,
            parent_classes: vec![],
            fields: vec![],
            functions: vec![],
        }
    }

    pub fn namespace(&self) -> &CSNamespace {
        unsafe { self.namespace.as_ref().unwrap() }
    }
}

impl Display for CSClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Into<CSCode> for CSClass {
    fn into(self) -> CSCode {
        todo!()
    }
}

/**********************************
*           CSFunction            *
**********************************/

#[derive(Clone, Debug)]
pub struct CSFunction {
    name : String,
    pub access : AccessModifier,
    pub arguments : Vec<(String, CSType)>,

    pub body : Vec<CSIntruction>,

    pub return_value : CSType,

    pub is_override : bool,
    pub scoped_variables : Vec<(String, CSType)>
}

impl CSFunction {
    pub fn new(name : String, args : Vec<(String, CSType)>, ret : CSType, instructions: Vec<CSIntruction>) -> Self {
        CSFunction {
            name,
            access: AccessModifier::Public,
            arguments: args,
            body: instructions,
            return_value: ret,
            is_override: false,
            scoped_variables: vec![],
        }
    }

    pub fn make_override(mut arg : Self) -> Self {
        arg.is_override = true;
        arg
    }
    /*
    pub fn method(subject : CSClassReference, name : String, args : Vec<(String, CSType)>, ret : CSType, instructions: Vec<String>) -> Self {
        CSFunction {
            name,
            access: AccessModifier::Public,
            arguments: args,
            body: instructions.join(""),
            return_value: ret,
            is_override: false,
            scoped_variables: CS,
        }
    }*/

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Display for CSFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({} -> {})",
               self.name,
               {
                   let mut s : Vec<String> = vec!["".to_string()];
                   for (_, t) in &self.arguments {
                       s.push(t.to_string());
                   }
                   s.join(", ")
               },
               self.return_value
        )
    }
}

impl Into<CSCode> for CSFunction {
    fn into(self) -> CSCode {
        todo!()
    }
}
