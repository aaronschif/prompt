extern crate termion;
extern crate regex;
extern crate git2;

mod git;

use std::path::Path;
use std::env;

use termion::{color, style};
use regex::Regex;

use git::{format, Status};

fn virtualenv() -> String {
    let r = Regex::new(r"([^/]*)/[^/]*$").unwrap();
    match env::var("VIRTUAL_ENV") {
        Ok(virt) => format!("PY{} ", r.captures(virt.as_str()).unwrap().get(1).unwrap().as_str()).to_string(),
        Err(_) => "".to_string()
    }
}

fn main() {
    let path_repr: String = match Path::new(".").canonicalize() {
        Ok(path) => {
            let r = path.to_str().unwrap().to_string();
            match env::var("HOME") {
                Ok(home) => {
                    let mat = Regex::new(format!("^{}", home).as_str()).unwrap();
                    mat.replace(r.as_str(), "~").into_owned()
                },
                Err(_) => r
            }
        },
        Err(_) => "!".to_string(),
    };

    let git_repr: String = match Status::from_cwd() {
        Ok(status) => format!("GIT{} ", format(&status)),
        Err(_) => "".to_string(),
    };

    println!(
        "{bold}{}{}{g}{} {b}λ  {r}",
        virtualenv(),
        git_repr,
        path_repr,
        // g=color::Fg(color::Green),
        g=color::Fg(color::Rgb(51, 232, 29)),
        // b=color::Fg(color::Blue),
        bold=style::Bold,
        b=color::Fg(color::Rgb(0, 147, 255)),
        r=color::Fg(color::Reset),
    );
}
