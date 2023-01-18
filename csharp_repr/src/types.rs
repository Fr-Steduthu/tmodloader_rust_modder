use std::fmt::{Display, Formatter};
use crate::project::{CSNamespace};

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
    pub fn prefix(&self) -> Option<CSTypePrefix> {
        self.prefix.clone()
    }
    pub fn cstype(&self) -> CSType {
        self.ty.clone()
    }
    pub fn is_array(&self) -> bool {
        self.is_array.clone()
    }

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

/**********************************
*             CSValue             *
**********************************/

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum CSRValue {
    Litteral(String, CSType),
    LValue(CSLValue),
    FuncCall(CSFunctionTerm, Vec<Box<CSRValue>>)
}

impl CSRValue {
    pub(crate) fn cstype(&self) -> CSType {
        match self {
            CSRValue::Litteral(_, t) => t.clone(),
            CSRValue::LValue(v) => v.cstype.clone(),
        }
    }

}
impl Display for CSRValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CSRValue::Litteral(n, _) => n.clone(),
            CSRValue::LValue(v) => v.to_string(),
        })
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSLValue {
    pub(crate) mutable : bool,
    identifier: String,
    cstype: CSType,
}

impl CSLValue {
    pub(crate) fn id(&self) -> String {
        self.identifier.clone()
    }
    pub(crate) fn cstype(&self) -> CSType {
        self.cstype.clone()
    }
}
impl Display for CSLValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum CSFunctionTerm {
    Function(CSMethod),
    ExternalFunction(String, Vec<CSType>, CSType),
}

impl Display for CSFunctionTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CSFunctionTerm::Function(func) => write!(f, "{func}"),
            CSFunctionTerm::ExternalFunction(name, argsty, ret) => {
                write!(f, "{}({} -> {})",
                   name,
                   {
                       let mut s : Vec<String> = vec!["".to_string()];
                       for t in argsty {
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

/**********************************
*          CSInstruction          *
**********************************/

#[derive(Clone, Debug)]
pub enum CSIntruction {
    Declaration(String, CSType, CSLValue),
    Affect(CSLValue, CSRValue),
    AffectToCall(CSLValue, CSFunctionTerm, Vec<CSRValue>),
    Call(CSFunctionTerm, Vec<CSRValue>),
    Return(CSRValue),
}

impl CSIntruction {
    pub fn declare(variable : &str, cstype : CSType, value : CSIntruction) -> Self {
        todo!("implement functions for CSInstruction")
    }
}

impl Display for CSIntruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   CSIntruction::Declaration(name, ty, val) => format!("let {name} : {ty} = {val};"),
                   CSIntruction::Affect(lval, rval) => format!("{lval} = {rval};"),
                   CSIntruction::Call(fun, args) => todo!("Display for CSIntruction::Call"),
                   CSIntruction::Return(rval) => todo!("Display for CSIntruction::Return"),
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

    pub fields : Vec<(String, AccessModifier, CSType)>,
    pub functions : Vec<(AccessModifier, CSMethod)>,
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

    pub fn inherits(&mut self, parent : CSClass) {
        self.parent_classes.push(parent.name());
        todo!("Inherit fields /!\\ remember to mind the accessibility")
    }

    pub fn new_field(&mut self, f : (String, AccessModifier, CSType)) {
        self.fields.push(f)
    }
    pub fn new_method(&mut self, name : &str, access : AccessModifier) -> &mut CSMethod {
        let mut f = CSMethod::new(
            self,
            name.to_string(), // Function name
            vec![], // Args
            CSType::void(), // Return type
            vec![] // Instructions
        );

        todo!("Creation des methodes");

        self.functions.push((access, f));
        match self.functions.last_mut().unwrap() { (_, m) => m }
    }
    pub fn new_override(&mut self, /*ARGS*/) {
        todo!("Create new method (override)")
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn method(&self, name : &str) -> &CSMethod {
        todo!("Implement CSClass::method(String)")
    }

    pub fn namespace(&self) -> &CSNamespace {
        unsafe { self.namespace.as_ref().unwrap() }
    }
}
impl Display for CSClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

/**********************************
*           CSFunction            *
**********************************/

#[derive(Clone, Debug)]
pub struct CSMethod {
    container : *const CSClass,

    name : String,

    arguments : Vec<(String, CSType)>,
    body : Vec<CSIntruction>,
    return_value : CSType,

    is_static : bool,
    is_override : bool,
}

impl CSMethod {
    pub fn new(parent : *const CSClass, name : String, args : Vec<(String, CSType)>, ret : CSType, instructions: Vec<CSIntruction>) -> Self {
        CSMethod {
            container: parent,

            name,

            arguments: args,
            body: instructions,
            return_value: ret,

            is_static: false,
            is_override: false,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn arguments(&self) -> Vec<(String, CSType)> {
        self.arguments.clone()
    }

    pub fn body(&self) -> Vec<CSIntruction> {
        self.body.clone()
    }
    pub fn body_mut(&mut self) -> &mut Vec<CSIntruction> {
        self.body.as_mut()
    }

    pub fn return_type(&self) -> CSType {
        self.return_value.clone()
    }

    pub fn make_static(&mut self) -> &mut Self {
        self.is_static = true;
        self
    }
    pub fn make_override(&mut self) -> &mut Self {
        self.is_override = true;
        self
    }

    pub fn is_override(&self) -> bool {
        self.is_override
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
}
impl Display for CSMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({} -> {})",
               self.name,
               {
                   let mut s : Vec<String> = vec!["(".to_string()];
                   for (_, t) in &self.arguments {
                       s.push(t.to_string());
                   }
                   s.push(")".to_string());
                   s.join(", ")
               },
               self.return_value
        )
    }
}
