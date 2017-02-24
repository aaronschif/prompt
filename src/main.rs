extern crate termion;
extern crate regex;
extern crate git2;
#[macro_use]
extern crate clap;

mod git;

use std::path::Path;
use std::env;

use clap::{App, Arg, SubCommand, ArgMatches};
use termion::{color, style};
use regex::Regex;

use git::{format, Status};

fn virtualenv() -> Option<String> {
    let r = Regex::new(r"([^/]*)/[^/]*$").unwrap();
    match env::var("VIRTUAL_ENV") {
        Ok(virt) => Some(r.captures(&virt).unwrap().get(1).unwrap().as_str().to_string()),
        Err(_) => None,
    }
}

fn path() -> Option<String> {
    match Path::new(".").canonicalize() {
        Ok(path) => {
            let r = path.to_str().unwrap().to_string();
            match env::var("HOME") {
                Ok(home) => {
                    let mat = Regex::new(format!("^{}", home).as_str()).unwrap();
                    // Some(mat.replace(&r, "~").into_owned())⌂
                    Some(mat.replace(&r, "⌂").into_owned())
                },
                Err(_) => Some(r)
            }
        },
        Err(_) => None
    }
}

fn sc_prompt(app: &ArgMatches) {
    let bold = format!("%{{{}%}}", style::Bold);
    let reset = format!("%{{{}{}%}}", color::Fg(color::Reset), style::Reset);
    let fg = format!("%{{{}%}}", color::Fg(color::Rgb(0, 147, 255)));
    let bg = format!("%{{{}%}}", color::Fg(color::Rgb(51, 232, 29)));

    let mut result: String = "".to_string();

    result.push_str(&bold);

    match app.value_of("lasterror") {
        Some(last_error) if last_error != "0" =>
            result.push_str(&format!("{}?{}{} ", fg, bg, last_error)),
        _ => {}
    }

    if let Some(virt_repr) = virtualenv() {
        result.push_str(&format!("{}PY{}{} ", fg, bg, virt_repr));
    }

    if let Ok(status) = Status::from_cwd() {
        result.push_str(&format!("{}GIT{}{} ", fg, bg, format(&status)));
    };

    if result.len() > 80 {
        result.push_str("\n");
    }

    result.push_str(&bg);
    result.push_str(match path() {Some(ref p) => &p, None => "!"});
    result.push_str(" ");
    result.push_str(&fg);
    // result.push_str("∴ ");
    result.push_str("λ ");
    result.push_str(&reset);

    print!("{}", result);
}

fn sc_init(app: &ArgMatches) {
    println!(r#"PROMPT="\$({} prompt --last-error \$?)""#,
        Path::new(&env::args().nth(0).unwrap()).canonicalize().unwrap().to_str().unwrap());
}

fn main() {

    let app = App::new("prompt")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Prints your prompt.")
        .subcommand(SubCommand::with_name("prompt")
            .arg(Arg::with_name("lasterror")
                .takes_value(true)
                .value_name("last error")
                .long("last-error")
                .help("-")))
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    if let Some(matches) = app.subcommand_matches("prompt") {
        sc_prompt(&matches);
    } else if let Some(matches) = app.subcommand_matches("init") {
        sc_init(&matches);
    }
}
