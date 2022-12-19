use crate::ToCS;
use crate::types::{CSClass};

lazy_static::lazy_static! {
    static ref NAMESPACE_NAME_REGEX : regex::Regex = regex::Regex::new("[a-z|A-Z]([a-z|_|A-Z|0-9]*)((::[a-z|A-Z]([a-z|_|A-Z|0-9]*))*)").unwrap();
}

#[derive(Clone, Debug)]
pub struct CSFile {
    name : String,
    extenssion : String,
    content : Vec<String>,
}

impl From<CSProject> for CSFile {
    fn from(value: CSProject) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct CSProject {
    name : String,
    namespaces : Vec<CSNamespace>,
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
}

impl ToCS for CSProject {
    fn to_cs(self) -> String {
        todo!()
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CSNamespace {
    project : * const CSProject,
    name : String,
    classes : Vec<CSClass>,
}

impl CSNamespace {
    fn new(name : &str, project : *const CSProject) -> Self {
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
        unsafe { self.project.as_ref() }.unwrap()
    }

    pub fn new_class(&mut self, name : &str) -> &mut CSClass {
        let cl = CSClass::new(name, self);
        self.classes.push(cl);
        self.classes.last_mut().unwrap()
    }
}

impl ToCS for CSNamespace {
    fn to_cs(self) -> String {
        todo!()
    }
}