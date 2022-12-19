use crate::project::CSNamespace;

#[path = "./trait_impls.rs"] pub mod trait_impls;
#[path = "./methods.rs"] mod methods;


#[derive(Clone, Debug)]
pub enum AccessModifier {
    Private,
    Protected,
    Public,
}

/**********************************
*             CSTYPE              *
**********************************/

#[derive(Clone, Debug)]
pub struct CSValue {
    pub ident : String,
    pub t : CSType,
}

#[derive(Clone, Debug)]
pub struct CSType {
    prefix : Option<CSTypePrefix>,
    t : CSPrimalType,
    is_array: bool,
}

#[derive(Clone, Debug)]
pub(crate) enum CSTypePrefix{ Ref, Out, Params}

#[derive(Clone, Debug)]
pub(crate) enum CSPrimalType {
    String,
    Integer,
    Float,
    Void,
    Class(String),
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

