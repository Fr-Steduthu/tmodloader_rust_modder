use std::fmt::Display;

pub mod tmod_types;
pub mod terraria_defaults;
pub mod item;

pub trait CSTemplate
{
    fn to_cs(self) -> String;
}

impl<T> CSTemplate for T where T : Display {
    fn to_cs(self) -> String {
        self.to_string()
    }
}


/** From the string_concat crate */
#[doc(hidden)]
#[macro_export]
macro_rules! concat_cs_code {
    ( $( $x:expr ),* ) => {
        {
            let mut s : String = String::new();
            $(s.push_str($x.to_cs().as_str());)*
            s
        }
    };
}