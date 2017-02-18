extern crate termion;
extern crate regex;
extern crate git2;

mod git;

use std::path::Path;
use std::env;

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
                    Some(mat.replace(&r, "~").into_owned())
                },
                Err(_) => Some(r)
            }
        },
        Err(_) => None
    }
}

fn main() {
    let bold = format!("%{{{}%}}", style::Bold);
    let reset = format!("%{{{}%}}", color::Fg(color::Reset));
    let fg = format!("%{{{}%}}", color::Fg(color::Rgb(0, 147, 255)));
    let bg = format!("%{{{}%}}", color::Fg(color::Rgb(51, 232, 29)));

    let mut result: Vec<String> = vec![bold];

    if let Ok(status) = Status::from_cwd() {
        result.push(format!("{}GIT{}{} ", fg, bg, format(&status)));
    };

    if let Some(virt_repr) = virtualenv() {
        result.push(format!("{}PY{}{} ", fg, bg, virt_repr));
    }

    result.push("\n".to_string());

    result.push(bg);
    result.push(match path() {Some(p) => p, None => "!".to_string()});
    result.push(" ".to_string());
    result.push(fg);
    result.push("∴ ".to_string());
    result.push(reset);

    print!("{}", result.join(""));
}
