use crate::project::{CSProject, CSNamespace, CSCode};
use crate::types::CSClass;

use file_repr::{File, Folder, FolderContent};

impl From<CSProject> for Folder {
    fn from(value: CSProject) -> Self {
        let mut fold = Folder::new(value.name());
        for ns in value.namespaces() {
            fold.add(ns.into());
        }

        fold
    }
}

impl From<CSNamespace> for Folder {
    fn from(value: CSNamespace) -> Self {
        let mut fold : Folder = Folder::new(value.name());
        for class in value.classes {
            fold.add(class.into())
        }

        fold
    }
}

impl From<CSClass> for File {
    fn from(value: CSClass) -> Self {
        todo!("Implement From<CSClass> for File")
    }
}