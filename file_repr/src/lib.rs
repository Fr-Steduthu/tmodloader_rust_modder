use crate::FolderContent::{Folder as FCFolder, File as FCFile};

#[derive(Clone, Debug)]
pub struct Folder {
    name : String,
    files : Vec<Box<FolderContent>>,
}

#[derive(Clone, Debug)]
pub enum FolderContent {
    Folder(Folder),
    File(File),
}

#[derive(Clone, Debug)]
pub struct File {
    pub name : String,
    pub content : String,
    pub ext : String,
}

impl Folder {
    pub fn new(name : String) -> Self {
        Folder {
            name,
            files: vec![],
        }
    }

    pub fn init(name : String, vec : Vec<FolderContent>) -> Folder {
        let mut this = Self::new(name);

        for fc in vec {
            this.add(fc);
        }

        this
    }

    pub fn add(&mut self, fc : FolderContent) {
        self.files.push(Box::new(fc))
    }

    pub fn save(&self, path : &str) {
        todo!()
    }

    pub fn load(&self, path : &str) {
        todo!()
    }
}

impl From<Folder> for FolderContent {
    fn from(value: Folder) -> Self {
        FCFolder(value)
    }
}

impl From<File> for FolderContent {
    fn from(value: File) -> Self {
        FCFile(value)
    }
}