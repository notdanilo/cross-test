#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use tokio::test as test;
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use wasm_bindgen_test::wasm_bindgen_test_configure as web_configure;
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
    web_configure!(run_in_browser);
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
