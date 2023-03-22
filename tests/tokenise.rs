use irctokens::Line;

#[test]
fn basic() {
    let line =
        Line::tokenise(b"@tag1=tag1value;tag2=;tag3 :source COMMAND arg1 arg2 :arg3 with space")
            .unwrap();

    assert_eq!(line.source, Some(b"source".to_vec()));
    assert_eq!(&line.command, "COMMAND");

    assert_eq!(line.args.len(), 3);
    assert_eq!(line.args[0], b"arg1");
    assert_eq!(line.args[1], b"arg2");
    assert_eq!(line.args[2], b"arg3 with space");

    let tags = line.tags.unwrap();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags["tag1"], Some("tag1value".to_string()));
    assert_eq!(tags["tag2"], None);
    assert_eq!(tags["tag3"], None);
}

#[test]
fn complex_tags() {
    let line = Line::tokenise(b"@tag1=a\\:a COMMAND").unwrap();

    let tags = line.tags.unwrap();
    assert_eq!(tags["tag1"], Some("a;a".to_string()));
}
