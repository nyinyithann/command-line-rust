#[test]
fn ls_test() {
    let mut cmd = std::process::Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn hello_test() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();    
    cmd.assert().success().stdout("hello\n");
}
