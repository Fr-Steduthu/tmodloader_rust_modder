use std::fmt::{Display, Formatter};
use file_repr::{Folder, FolderContent};
use crate::{
    CSCode,
    types::{
        CSClass
    }
};

/**********************************
*            CSProject            *
**********************************/

#[derive(Clone, Debug)]
pub struct CSProject {
    name : String,
    namespaces : Vec<CSNamespace>,
}

impl CSProject {
    pub fn new(name : &str) -> Self {
        CSProject {
            name : name.split_whitespace().collect::<Vec<&str>>().join("").to_string(),
            namespaces: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn new_namespace(&mut self, name : &str) {
        let namespace = CSNamespace::new(name, self);
        self.namespaces.push(namespace.unwrap());
    }
    pub fn get_new_namespace(&mut self, name : &str) -> &mut CSNamespace {
        self.new_namespace(name);
        self.namespaces.last_mut().unwrap()
    }

    pub fn namespace(&self, name : &str) -> Result<&CSNamespace, String> {
        if !NAMESPACE_NAME_REGEX.is_match(name) {
            return Err(format!("{name} is not a valid namespace name"))
        }

        for s in self.namespaces.iter() {
            if s.name == name {
                return Ok(s);
            }
        }

        Err(format!("namespace of long name \"{name}\" not found in project \"{}\"", self.name))
    }
    pub fn namespace_mut(&mut self, name : &str) -> Result<&mut CSNamespace, String> {
        if !NAMESPACE_NAME_REGEX.is_match(name) {
            return Err(format!("{name} is not a valid namespace name"))
        }

        for ns in self.namespaces.iter_mut() {
            if ns.name == name {
                return Ok(ns);
            }
        }
        Err(format!("namespace of long name \"{name}\" not found in project \"{}\"", self.name))
    }

    pub fn namespaces(&self) -> Vec<CSNamespace> {
        self.namespaces.clone()
    }
}

/**********************************
*           CSNamespace           *
**********************************/

lazy_static::lazy_static! { pub(crate) static ref NAMESPACE_NAME_REGEX : regex::Regex = regex::Regex::new(r"(?:@?([a-zA-Z]\w)*(?:\.@?[a-zA-Z_]\w*)+)").unwrap(); }

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSNamespace {
    pub(crate) project : * const CSProject,
    pub(crate) name : String,
    pub(crate) classes : Vec<CSClass>,
}

impl CSNamespace {

    pub fn new(name : &str, project : * const CSProject) -> Result<Self, String> {
        if NAMESPACE_NAME_REGEX.is_match(name) {
            Ok(CSNamespace {
                project,
                name : name.to_string(),
                classes: vec![],
            })
        } else {
            Err(format!("{name} is not a valid namespace name"))
        }
    }

    pub fn project(&self) -> &CSProject {
        unsafe { self.project.as_ref() }.unwrap()
    }

    pub fn new_class(&mut self, name : &str) -> &mut CSClass {
        self.classes.push(CSClass::new(name, self));
        self.classes.last_mut().unwrap()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn class(&self, class_name : &str) -> &CSClass {
        for c in self.classes.iter() {
            if class_name.to_string() == c.name() {
                return c
            }
        }
    }
    pub fn class_mut(&mut self, class_name : &str) -> &mut CSClass {
        for c in self.classes.iter_mut() {
            if class_name.to_string() == c.name() {
                return c
            }
        }
    }
}