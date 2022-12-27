use std::fmt::{Display, Formatter};

use crate::project::CSNamespace;
use crate::types::_CSType::Void;

#[path = "./trait_impls.rs"] pub mod trait_impls;
#[path = "./methods.rs"] mod methods;

/**********************************
*             CSTYPE              *
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

#[derive(Clone, Debug)]
pub enum CSValue {
    Variable(String, CSType),
    Litteral(String, CSType),
    Function(CSFunction),
    ExternalFunction(String, Vec<CSType>, CSType),
}

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
                for (_, t) in f.arguments {
                    ret.push(t);
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
                "".to_string()
            }
            CSIntruction::Affect(lvalue, rvalue) => { vec![lvalue.to_string(), " = ".to_string(), rvalue.to_string()].join("") }
            CSIntruction::Value(v) => { format!("return {};", v.to_string()) }
        })
    }
}

impl CSIntruction {
    pub(crate) fn get_type(self) -> Result<CSType, String> {
        match self {
            CSIntruction::Call(func, arg_values) => {
                if !func.is_function() {
                    return Err(format!("{} is not a function", func.identifier()));
                }

                for (arg_v, arg_t) in arg_values.zip(func.arguments().unwrap()) {
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

                Ok(Void)
            }
            CSIntruction::Value(csvalue) => { Ok(csvalue.cstype()) }
        }
    }
}

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


/**********************************
*             CSCLASS             *
**********************************/

#[derive(Clone, Debug)]
pub struct CSClass {
    pub name : String,
    namespace : * const CSNamespace,
    pub accessibility : AccessModifier,

    pub parent_classes : Vec<String>,

    pub fields : Vec<(String, CSType)>,
    pub functions : Vec<CSFunction>,
}

#[derive(Clone, Debug)]
pub struct CSFunction {
    pub name : String,
    pub access : AccessModifier,
    pub arguments : Vec<(String, CSType)>,

    pub body : String,

    pub return_value : CSType,

    pub is_override : bool,
    pub scoped_variables : Vec<(String, CSType)>
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