use std::io;
use std::io::{Read, Write};
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
    name : String,
    content : String,
    ext : String,
}

impl Folder {
    pub fn new(name : String) -> Self {
        Self::init(name, vec![])
    }
    pub fn init(name : String, folder_contents : Vec<FolderContent>) -> Self {
        Folder {
            name,
            files: {
                let mut fc = vec![];
                for fcelem in folder_contents {
                    fc.push(Box::new(fcelem));
                }
                fc
            },
        }
    }

    pub fn add(&mut self, fc : FolderContent) {
        self.files.push(Box::new(fc))
    }
    pub fn new_folder(&mut self, name : String) -> &mut Folder {
        self.add(FCFolder(Folder::new(name.to_string())));
        if let FCFolder(folder) = self.files.last_mut().unwrap().as_mut() {
            folder
        } else {
            panic!("Dead code reached")
        }
    }
    pub fn new_file(&mut self, name : &str, extenssion : &str) -> &mut File {
        self.add(FCFile(File::new(name, extenssion)));
        if let FCFile(file) = self.files.last_mut().unwrap().as_mut() {
            file
        } else {
            panic!("Dead code reached")
        }
    }

    pub fn save(&self, path : &str) -> Result<(), std::io::Error>{
        for fc in self.files.iter() {
            fc.save([path, "/", self.name.as_str()].join("").as_str())?;
        }
        Ok(())
    }
    pub fn load(&self, path : &str) -> io::Result<Folder> {
        todo!()
    }
}
impl File {
    pub fn new(name : &str, extenssion : &str) -> Self {
        File {
            name: name.to_string(),
            content: "".to_string(),
            ext: extenssion.to_string(),
        }
    }

    pub fn append(&mut self, content : &str) {
        self.content.push_str(content)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn content_mut(&mut self) -> &mut str {
        self.content.as_mut()
    }

    pub fn extenssion(&self) -> String {
        self.ext.clone()
    }

    pub fn save(&self, path : &str) -> Result<(), std::io::Error> {
        let mut fd = std::fs::File::create(format!("{path}.{}", self.ext).as_str()).unwrap();
        fd.write(self.content.as_bytes())?;
        Ok(())
    }
    pub fn load(&self, path : &str) -> io::Result<Self> {
        let mut fd = std::fs::File::open(path)?;
        let (name, ext) = path.split(|c| { c == '\\' || c == '/' }).last().unwrap().split_once(".").unwrap();
        Ok(File{
            name : name.to_string(),
            content: unsafe {
                let mut slice = vec![];
                let nb_bytes = fd.read(slice.as_mut_slice())?;
                String::from_raw_parts(slice.as_mut_slice().as_mut_ptr(), nb_bytes, nb_bytes)
            },
            ext : ext.to_string(),
        })
    }
}

impl FolderContent {
    pub fn save(&self, path : &str) -> Result<(), std::io::Error> {
        match self {
            FolderContent::Folder(f) => f.clone().save(path),
            FolderContent::File(f) => f.clone().save(path),
        }
    }
    pub fn load(&self, path : &str) -> io::Result<Self> {
        match self {
            FolderContent::Folder(f) => Ok(FCFolder(f.clone().load(path)?)),
            FolderContent::File(f) => Ok(FCFile(f.clone().load(path)?)),
        }
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

impl TryFrom<FolderContent> for Folder {
    type Error = String;
    fn try_from(value: FolderContent) -> Result<Self, Self::Error> {
        match value {
            FCFile(file) => Err("Could not convert File to Folder".to_string()),
            FCFolder(folder) => Ok(folder),
        }
    }
}
impl TryFrom<FolderContent> for File {
    type Error = String;
    fn try_from(value: FolderContent) -> Result<Self, Self::Error> {
        match value {
            FCFolder(folder) => Err("Could not convert Folder to File".to_string()),
            FCFile(file) => Ok(file),
        }
    }
}