use crate::ToCS;
use crate::types::{CSClass};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSProject {
    pub namespaces : Vec<CSNamespace>,
}

impl ToCS for CSProject {
    fn to_cs(self) -> String {
        todo!()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CSNamespace {
    pub classes : Vec<CSClass>,
}

impl ToCS for CSNamespace {
    fn to_cs(self) -> String {
        todo!()
    }
}