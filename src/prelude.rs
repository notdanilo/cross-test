#[cfg(not(target_arch = "wasm32"))]
pub use tokio;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test;

pub use crate::configure as cross_test_configure;
pub use crate::test as cross_test;