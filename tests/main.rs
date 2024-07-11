mod util;

use assert_cmd::Command;
use util::TestContext;

#[test]
fn test_docs_example() {
    let ctx = TestContext::new("test_docs_example");
    let mut cmd = Command::cargo_bin("stamp").unwrap();
    cmd.args(&[&ctx.source, &ctx.result]);
    cmd.assert().success();
    ctx.assert();
}
