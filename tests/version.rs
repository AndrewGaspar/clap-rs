extern crate clap;
extern crate regex;

use std::str;

use clap::{App, ErrorKind};

include!("../clap-test.rs");

static VERSION: &'static str = "clap-test v1.4.8";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "-V"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "--version"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn complex_version_output() {
    let mut a = App::new("clap-test").version("v1.4.8");
    let _ = a.get_matches_from_safe_mut(vec![""]);

    // Now we check the output of print_version()
    let mut ver = vec![];
    a.write_version(&mut ver).unwrap();
    assert_eq!(str::from_utf8(&ver).unwrap(), VERSION);
}

#[test]
fn override_ver() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .mut_arg("version", |a| a.short("v").help("some version"))
        .get_matches_from_safe(vec!["test", "-v"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}
