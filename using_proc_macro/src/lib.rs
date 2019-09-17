#[derive(proc_macro_crate::ProcMacro)]
struct Hello(
    &'a (),
    extern fn(),
);


#[derive(proc_macro_crate::ProcMacro)]
struct World(
    &'a (),
    extern "C" fn(),
);