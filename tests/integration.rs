use std::borrow::Cow;

#[test]
fn test() {
    let expected: &str = "Hello, world!";

    let output = std::process::Command::new("cargo")
    .arg("run")
        .output()
    .expect("failed to execute process");

    let binding: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let actual: &str = binding.trim();

    assert_eq!(expected, actual);
}
