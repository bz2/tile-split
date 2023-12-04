use super::*;

#[test]
fn arg_parse_range_invalud() {
    for s in ["", "-", " "] {
        let err = parse_range::<u8>(s).unwrap_err();
        assert_eq!(err.to_string(), "invalid range");
    }
    for s in ["  ", " -", "- ", "---"] {
        let err = parse_range::<u8>(s).unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }
    for s in ["fish", "something bogus"] {
        let err = parse_range::<u8>(s).unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }
    for s in ["1-0", "55-22"] {
        let err = parse_range::<u8>(s).unwrap_err();
        assert_eq!(err.to_string(), "invalid range");
    }
}

#[test]
fn arg_parse_range_ok() {
    assert_eq!(parse_range::<u8>("0").unwrap(), 0..=0);
    assert_eq!(parse_range::<u8>("3").unwrap(), 3..=3);
    assert_eq!(parse_range::<u8>("-4").unwrap(), 0..=4);
    assert_eq!(parse_range::<u8>("2-").unwrap(), 2..=255);
    assert_eq!(parse_range::<u8>("5-5").unwrap(), 5..=5);
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
