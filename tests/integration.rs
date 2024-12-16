use std::borrow::Cow;

#[test]
fn test() {
    let expected: &str = "\
A\n\
ACTUAL\n\
AGREE\n\
AGREEMENT\n\
AND";

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg("cargo run -- -u tests/words.txt | head -n5")
        .output()
    .expect("failed to execute process");

    let binding: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let actual: &str = binding.trim(); // Remove BOM character at the start

    assert_eq!(expected, actual);
}
