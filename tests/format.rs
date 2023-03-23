use irctokens::Line;
use std::collections::BTreeMap;

#[test]
fn basic() {
    let line = Line {
        tags: Some(BTreeMap::from([
            ("tag1".to_string(), Some("tag1value".to_string())),
            ("tag2".to_string(), None),
            ("tag3".to_string(), Some("a;a".to_string())),
        ])),
        source: Some(b"source".to_vec()),
        command: "COMMAND".to_string(),
        arguments: Vec::from([
            b"arg1".to_vec(),
            b"arg2".to_vec(),
            b"arg3 with space".to_vec(),
        ]),
    }
    .format();

    assert_eq!(
        line,
        b"@tag1=tag1value;tag2;tag3=a\\:a :source COMMAND arg1 arg2 :arg3 with space"
    );
}
