//! # cross-test
//! Run Rust native and web tests with a single framework.
//!
//! ### Platforms
//! * Web: web-sys
//! * Native: tokio
//!
//! ### Requirements
//!
//! To run the `wasm-bindgen-test`s, you need to have the nightly toolchain installed (there is no need to make it default).
//!
//! You will also need to manually include `wasm-bindgen-test` as a dev dependency in your project because `#[wasm_bindgen_test]` macro can't be re-exported to avoid this requirement because of how it was designed.
//!
//! ```
//! [target.'cfg(target_arch = "wasm32")'.dependencies]
//! wasm-bindgen-test = "0.3.20"
//! ```
//!
//! ### Usage
//! ```
//!use cross_test::prelude::*;
//!
//!cross_test_configure!();
//!
//!#[cross_test]
//!async fn it_works() {
//!    assert_eq!(2 + 2, 4);
//!}
//! ```
//!
//! ### Important issue
//!
//! Because `#[cross_test]` gets translated to `#[tokio::test]` all the tests must be `async`.
//!
//! A custom proc-macro will be provided to select the test executor if the `async` work is present or not.

#[warn(missing_docs)]

pub mod prelude;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use tokio::test as test;

    #[macro_export]
    macro_rules! configure {
        () => {}
    }
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use wasm_bindgen_test::wasm_bindgen_test as test;

    #[macro_export]
    macro_rules! configure {
        () => {
            wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
        }
    }
}

pub use platform::*;

#[cfg(test)]
mod tests {
    use crate as cross_test;
    use cross_test::prelude::*;

    cross_test_configure!();

    #[cross_test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
