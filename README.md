# cross-test
Run Rust native and web tests with a single framework.

### Platforms
* Web: web-sys
* Native: tokio

 ### Example
 ```rust
use cross_test::prelude::*;

cross_test_configure!();

#[cross_test]
async fn it_works() {
    assert_eq!(2 + 2, 4);
}
 ```

### Important issue

Because `#[cross_test]` gets translated to `#[tokio::test]` all the tests must be `async`.

A custom proc-macro will be provided to select the test executor if the `async` work is present or not.

[Link for the issue: https://github.com/notdanilo/cross-test/issues/1](https://github.com/notdanilo/cross-test/issues/1)
