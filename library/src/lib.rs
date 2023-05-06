
pub mod tmod_types;
pub mod terraria_defaults;
pub mod item;


pub mod cs
{
    pub trait CSTemplate
    {
        fn to_cs(self) -> String;
    }

    use std::fmt::Display;
    impl<T> CSTemplate for T where T : Display
    {
        fn to_cs(self) -> String
        {
            self.to_string()
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! concat_cs_code
    {
        ( $( $x:expr ),* ) =>
        {
            {
                let mut s : String = String::new();
                $(s.push_str($x.to_cs().as_str());)*
                s
            }
        };
    }
}