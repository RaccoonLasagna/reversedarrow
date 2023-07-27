use assert_cmd::Command;

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("HW4_3_arrow").unwrap();
    cmd.assert().success().stdout("Input a number!\n");
}

#[test]
fn length_5() {    
    let mut cmd = Command::cargo_bin("HW4_3_arrow").unwrap();
    cmd.arg("5").assert().success().stdout("    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n");
}