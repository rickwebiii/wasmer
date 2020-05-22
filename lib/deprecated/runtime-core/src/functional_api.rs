use crate::{module::Module, new};
use std::error::Error;

#[deprecated(
    since = "__NEXT_VERSION__",
    note = "Please use `Module::new(&store, bytes)` instead."
)]
pub fn compile_with(bytes: &[u8], _compiler: ()) -> Result<Module, Box<dyn Error>> {
    let store = Default::default();

    Ok(Module::new(new::wasmer::Module::new(&store, bytes)?))
}

#[deprecated(
    since = "__NEXT_VERSION__",
    note = "Please use `Module::new(&store, bytes)` instead."
)]
pub fn compile_with_config(
    bytes: &[u8],
    _compiler: (),
    _compiler_config: (),
) -> Result<Module, Box<dyn Error>> {
    let store = Default::default();

    Ok(Module::new(new::wasmer::Module::new(&store, bytes)?))
}

#[deprecated(
    since = "__NEXT_VERSION__",
    note = "Please use `Module::validate(&store, bytes)` instead."
)]
pub fn validate(bytes: &[u8]) -> bool {
    let store = Default::default();

    new::wasmer::Module::validate(&store, bytes).is_ok()
}
