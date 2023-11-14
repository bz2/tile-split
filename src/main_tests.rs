use super::*;

/// Parse args and panic on failure but without exiting the process
fn parse<'a, I>(arr: I) -> Args
where
    I: std::iter::IntoIterator<Item = &'a str>,
{
    Args::try_parse_from(arr).expect("arg parse failed")
}

#[test]
fn args_prompt_help_on_empty() {
    let err = Args::try_parse_from(["bin"]).expect_err("arg parse passed!?");
    let out = err.to_string();
    assert!(out.contains("--help"), "need --help in output {:?}", out);
}

#[test]
#[should_panic] // known failure, --zoomlevel is required
fn args_file_only() {
    let args = parse(["bin", "a-file.png"]);
    assert_eq!(args.filename, "a-file.png");
}

#[test]
fn args_file_and_level() {
    let args = parse(["bin", "-l", "7", "a-file.png"]);
    assert_eq!(args.filename, "a-file.png");
    assert_eq!(args.zoomlevel, 7);
}