use crate::project::CSNamespace;
use crate::types::CSClass;
use super::{CSType, CSFunction, CSPrimalType, AccessModifier};


impl CSType {
    pub fn void() -> Self {
        CSType {
            prefix: None,
            t: CSPrimalType::Void,
            is_array: false,
        }
    }

    pub fn integer() -> Self {
        CSType {
            prefix: None,
            t: CSPrimalType::Integer,
            is_array: false,
        }
    }

    pub fn class(name : String) -> Self {
        CSType {
            prefix: None,
            t: CSPrimalType::Class(name),
            is_array: false,
        }
    }
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


impl CSFunction {
    pub fn new(name : String, args : Vec<(String, CSType)>, ret : CSType, instructions: Vec<String>) -> Self {
        CSFunction {
            name,
            access: AccessModifier::Public,
            arguments: args,
            body: instructions.join(""),
            return_value: ret,
            is_override: false,
            scoped_variables: vec![],
        }
    }

    pub fn ovrd(mut arg : Self) -> Self {
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
}