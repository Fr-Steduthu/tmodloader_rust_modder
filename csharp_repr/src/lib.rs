pub mod types;
pub mod project;

pub trait ToCS {
    fn to_cs(self) -> String;
}