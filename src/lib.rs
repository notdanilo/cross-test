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
    use crate::test;

    #[test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
