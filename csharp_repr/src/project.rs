use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::types::{CSClass};

lazy_static::lazy_static! {
    pub(crate) static ref NAMESPACE_NAME_REGEX : regex::Regex = regex::Regex::new(r"(?:@?([a-zA-Z]\w)*(?:\.@?[a-zA-Z_]\w*)+)").unwrap();
}

//#[doc(hidden)]
pub struct CSCode {
    content : String,
}

impl ToString for CSCode {
    fn to_string(&self) -> String {
        self.content.clone()
    }
}
impl From<String> for CSCode {
    fn from(value: String) -> Self {
        CSCode{ content: value.clone() }
    }
}
impl Display for CSCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.clone())
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSFile {
    pub(crate) name : String,
    pub(crate) extenssion : String,
    pub(crate) content : Vec<CSCode>,
}

impl From<CSProject> for Vec<CSFile> {
    fn from(value: CSProject) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct CSProject {
    pub(crate) name : String,
    pub(crate) namespaces : Vec<CSNamespace>,
}

impl CSProject {
    pub fn new(name : &str) -> Self {
        CSProject {
            name : name.to_string(),
            namespaces: vec![],
        }
    }

    pub fn new_namespace(&mut self, name : &str) -> &mut CSNamespace {
        let namespace = CSNamespace::new(name, self);
        self.namespaces.push(namespace);
        self.namespaces.last_mut().unwrap()
    }

    pub fn new_nested_namespace(&mut self, parent : CSNamespace, ns : String) -> &mut CSNamespace {
        self.new_namespace([parent.name, ns].join(".").as_str())
    }

    pub fn namespace(&self, name : String) -> Result<&CSNamespace, String> {
        for s in self.namespaces.iter() {
            if s.name == name {
                return Ok(s);
            }
        }

        Err(format!("namespace of long name \"{name}\" not found in project \"{}\"", self.name))
    }

    pub fn namespace_mut(&mut self, name : String) -> Result<*mut CSNamespace, String> {
        for index in 0..self.namespaces.len() {
            let ns = self.namespaces.get_mut(index).unwrap();
            if ns.name == name {
                return Ok(ns);
            }
        }
        Err(format!("namespace of long name \"{name}\" not found in project \"{}\"", self.name))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Into<Vec<CSFile>> for CSProject {
    fn into(self) -> Vec<CSFile> {
        let v : Vec<CSFile> = vec![];

    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSNamespace {
    pub(crate) project : * mut CSProject,
    pub(crate) name : String,
    pub(crate) classes : Vec<CSClass>,
}

impl CSNamespace {
    fn new(name : &str, project : * mut CSProject) -> Self {
        if NAMESPACE_NAME_REGEX.is_match(name) {
            CSNamespace {
                project,
                name : name.to_string(),
                classes: vec![],
            }
        } else {
            panic!("{}", name)
        }
    }

    pub fn project(&self) -> &CSProject {
        unsafe { self.project.as_mut() }.unwrap()
    }

    pub fn new_class(&mut self, name : &str) -> &mut CSClass {
        let cl = CSClass::new(name, self);
        self.classes.push(cl);
        self.classes.last_mut().unwrap()
    }
}

impl Into<Vec<CSFile>> for CSNamespace {
    fn into(self) -> Vec<CSFile> {
        let mut v = vec![];
        for class in self.classes {
            v.push(class.into::<CSFile>())
        }
        v
    }
}