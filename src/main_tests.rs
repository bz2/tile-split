use super::*;

#[test]
fn arg_parse_range_empty() {
    // TODO: could support partially open ranges and default to 0-Idx::MAX
    for s in ["", "  ", "4-", "-4"] {
        let err = parse_range::<u8>(s).unwrap_err();
        assert_eq!(err.to_string(), "cannot parse integer from empty string");
    }
}

#[test]
fn arg_parse_range_ok() {
    assert_eq!(parse_range::<u8>("0").unwrap(), 0..=0);
    assert_eq!(parse_range::<u8>("3").unwrap(), 3..=3);
}

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
    assert_eq!(args.filename.as_path().display().to_string(), "a-file.png");
}

#[test]
fn all_args() {
    let args = parse(["bin", "-l", "7", "a-file.png", "-r", "0-5", "-t", "0-333"]);
    assert_eq!(args.filename.as_path().display().to_string(), "a-file.png");
    assert_eq!(args.zoomlevel, 7);
    assert_eq!(args.zoomrange, 0..=5);
    assert_eq!(args.targetrange, Some(0..=333));
}
