use crate::csharp_repr;

thread_local! {
    static Project : csharp_repr::project::CSProject = csharp_repr::project::CSProject::new();
}

const DefaultModItemNamespace : CSNamespace = CSNamespace::new(unsafe { Project }, "MyItems");