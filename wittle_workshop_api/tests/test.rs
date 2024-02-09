use std::process::Command;

#[test]
fn check_that_the_ls_command_works() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}