use iouring_restrict_systemd::{render_dropin, validate_dropin, Policy};
use std::path::Path;

#[test]
fn default_profile_dropin_roundtrip() {
    let p = Policy::parse_json(include_str!("../fixtures/default.json")).unwrap();
    assert_eq!(p.ops.len(), 38);
    let text = render_dropin(
        Path::new("fixtures/default.json"),
        Path::new("target/debug/iouring-restrict"),
        "lab",
    );
    validate_dropin(&text).unwrap();
    assert!(text.contains("fixtures/default.json"));
    assert!(text.contains("--fail-without-kernel-support"));
}

#[test]
fn fifty_dropin_generations_stable() {
    let mut last = String::new();
    for i in 0..50 {
        let t = render_dropin(
            Path::new("/etc/iouring/default.json"),
            Path::new("/usr/bin/iouring-restrict"),
            "stability",
        );
        validate_dropin(&t).unwrap();
        if i > 0 {
            assert_eq!(t, last);
        }
        last = t;
    }
}
