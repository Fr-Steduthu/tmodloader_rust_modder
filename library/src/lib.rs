
pub mod tmod_types;
pub mod terraria_defaults;



pub mod cs
{
    use std::fmt::Display;

    pub trait IntoCSCode
    {
        fn into_cs(self) -> String;
    }

    trait _AutoIntoCSCodeMarker {}

    impl _AutoIntoCSCodeMarker for String {}
    impl<'a> _AutoIntoCSCodeMarker for &'a str {}
    impl _AutoIntoCSCodeMarker for i64 {}
    impl _AutoIntoCSCodeMarker for u16 {}
    impl _AutoIntoCSCodeMarker for bool {}
    impl _AutoIntoCSCodeMarker for u64 {}
    impl _AutoIntoCSCodeMarker for u32 {}
    impl _AutoIntoCSCodeMarker for i8 {}

    impl<T : _AutoIntoCSCodeMarker + Display> IntoCSCode for T
    {
        fn into_cs(self) -> String {
            self.to_string()
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! concat_cs_code
    {
        () => {};
        ( $( $x:expr ),* ) =>
        {
            {
                let mut s : String = String::new();
                $(s.push_str($x.clone().into_cs().as_str());)*
                s
            }
        }
    }
}