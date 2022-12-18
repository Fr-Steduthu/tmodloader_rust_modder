#[path = "./trait_impls.rs"] pub mod trait_impls;
#[path = "./methods.rs"] mod methods;


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum AccessModifier {
    Private,
    Protected,
    Public,
}

/**********************************
*             CSTYPE              *
**********************************/

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSValue {
    pub ident : String,
    pub t : CSType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSType {
    prefix : Option<CSTypePrefix>,
    t : CSPrimalType,
    is_array: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub(crate) enum CSTypePrefix{ Ref, Out, Params}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSClass {
    pub classname : String,
    pub namespace : String,
    pub accessibility : AccessModifier,

    pub parents: Vec<String>,

    pub fields : Vec<(String, CSType)>,
    pub functions : Vec<CSFunction>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSFunction {
    pub name : String,
    pub access : AccessModifier,
    pub arguments : Vec<(String, CSType)>,

    pub body : String,

    pub return_value : CSType,

    pub is_override : bool,
    pub scoped_variables : Vec<(String, CSType)>
}

