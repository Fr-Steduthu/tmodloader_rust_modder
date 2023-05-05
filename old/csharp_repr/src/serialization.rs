use crate::project::{CSProject, CSNamespace};
use crate::types::CSClass;

use file_repr::{File, Folder};
use crate::CSCode;

impl From<CSProject> for Folder {
    fn from(value: CSProject) -> Self {
        let mut fold = Folder::new(value.name());
        for ns in value.namespaces() {
            fold.add(file_repr::Folder::from(ns).into());
        }

        fold
    }
}

impl From<CSNamespace> for Folder {
    fn from(value: CSNamespace) -> Self {
        let mut fold : Folder = Folder::new(value.name());

        for class in value.classes {
            fold.add(file_repr::File::from(class).into())
        }

        fold
    }
}

impl From<CSClass> for File {
    fn from(value: CSClass) -> Self {
        let mut f = file_repr::File::new(value.name().as_str(), ".cs");
        f.append(CSCode::from(value).to_string().as_str());
        f
    }
}