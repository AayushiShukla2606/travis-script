extern crate pulldown_cmark;
extern crate reqwest;

use pulldown_cmark::{Event, Parser, Tag};

use std::fs::File;
use std::io::Read;

fn fetch_rustfmt_readme() -> String {
    const URL: &str =
        "https://raw.githubusercontent.com/rust-lang-nursery/rustfmt/master/README.md";
    let mut resp = reqwest::get(URL).unwrap();
    assert!(resp.status().is_success(), "Could not fetch {}", URL);
    resp.text().unwrap()
}

fn get_remote_config() -> String {
    // Get the rustfmt README.
    let remote_config = fetch_rustfmt_readme();

    // Extract the config snippet from the README.
    let parser = Parser::new(&remote_config)
        .skip_while(|event| match *event {
            Event::Text(ref s) => s
                != "A minimal Travis setup could look like this (requires Rust 1.24.0 or greater):",
            _ => true,
        })
        .skip_while(|event| !matches!(*event, Event::Start(Tag::CodeBlock(_))))
        .take_while(|event| !matches!(*event, Event::End(Tag::CodeBlock(_))));

    let mut block = vec![];
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(attr)) => assert!(attr == "yaml"),
            Event::Text(s) => block.push(s.into_owned()),
            _ => panic!(),
        }
    }

    block.join("")
}

fn read_local_config() -> String {
    const FILENAME: &str = ".travis.yml";
    let mut f = File::open(FILENAME).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[test]
fn verify_local_remote_configs_match() {
    assert!(
        get_remote_config() == read_local_config(),
        "rustfmt Travis CI config does not match this repository's config"
    );
}
