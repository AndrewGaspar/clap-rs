extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, Arg, ErrorKind, ArgSettings};

static VISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help [aliases: dongle, done]";

static INVISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help";

#[cfg(feature = "suggestions")]
static DYM: &'static str = "error: The subcommand 'subcm' wasn't recognized
\tDid you mean 'subcmd'?

If you believe you received this message in error, try re-running with 'clap-test -- subcm'

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help";

#[cfg(feature = "suggestions")]
static DYM2: &'static str = "error: Found argument '--subcmdarg' which wasn't expected, or isn't valid in this context
\tDid you mean to put '--subcmdarg' after the subcommand 'subcmdarg'?

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help";

#[test]
fn subcommand() {
    let m = App::new("test")
        .subcommand(App::new("some").arg(Arg::new("test")
                                                          .short("t")
                                                          .long("test")
                                                          .set(ArgSettings::TakesValue)
                                                          .help("testing testing")))
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn subcommand_none_given() {
    let m = App::new("test")
        .subcommand(App::new("some").arg(Arg::new("test")
                                                          .short("t")
                                                          .long("test")
                                                          .set(ArgSettings::TakesValue)
                                                          .help("testing testing")))
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec![""]);

    assert!(m.subcommand_name().is_none());
}

#[test]
fn subcommand_multiple() {
    let m = App::new("test")
        .subcommands(vec![App::new("some").arg(Arg::new("test")
                                                                .short("t")
                                                                .long("test")
                                                                .set(ArgSettings::TakesValue)
                                                                .help("testing testing")),
                          App::new("add").arg(Arg::new("roster").short("r"))])
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert!(m.subcommand_matches("some").is_some());
    assert!(m.subcommand_matches("add").is_none());
    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn single_alias() {
    let m = App::new("myprog")
        .subcommand(App::new("test").alias("do-stuff"))
        .get_matches_from(vec!["myprog", "do-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn multiple_aliases() {
    let m = App::new("myprog")
        .subcommand(App::new("test").aliases(&["do-stuff", "test-stuff"]))
        .get_matches_from(vec!["myprog", "test-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output() {
    test::compare_output(test::complex_app(), "clap-test subcm", DYM, true);
    test::compare_output(test::complex_app(), "clap-test --subcmdarg foo", DYM2, true);
}

#[test]
fn alias_help() {
    let m = App::new("myprog")
        .subcommand(App::new("test").alias("do-stuff"))
        .get_matches_from_safe(vec!["myprog", "help", "do-stuff"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn visible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(App::new("test")
                        .about("Some help")
                        .alias("invisible")
                        .visible_alias("dongle")
                        .visible_alias("done"));
    test::compare_output(app, "clap-test --help", VISIBLE_ALIAS_HELP, false);
}

#[test]
fn invisible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(App::new("test")
                        .about("Some help")
                        .alias("invisible"));
    test::compare_output(app, "clap-test --help", INVISIBLE_ALIAS_HELP, false);
}
