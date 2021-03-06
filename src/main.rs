mod git;
mod providers;
mod formatter;
mod shells;

use std::path::Path;
use std::env;

use hostname::get_hostname;
use clap::{App, Arg, SubCommand, ArgMatches, arg_enum, _clap_count_exprs, value_t};
use termion::{color, style};
use regex::Regex;

use self::git::{format, Status};
use self::formatter::{Formatter};
use self::shells::{Zsh, Fish, Bash, Shell, Style};

arg_enum!{
    #[derive(PartialEq, Debug)]
    pub enum SupportedShells {
        Zsh,
        Bash,
        Fish,
    }
}

fn ssh_hostname() -> Option<String> {
    if let Ok(_) = env::var("SSH_CONNECTION") {
        if let Some(host) = get_hostname() {
            return Some(host.into());
        }
    }
    return None;
}

fn virtualenv() -> Option<String> {
    let r = Regex::new(r"([^/]*)/[^/]*$").unwrap();
    match env::var("VIRTUAL_ENV") {
        Ok(virt) => Some(r.captures(&virt).unwrap().get(1).unwrap().as_str().to_string()),
        Err(_) => None,
    }
}

fn path() -> Option<String> {
    let path = providers::PromptPath {
        home_string: Some("~".to_string()),
        seperator: "/".to_string(),
        shorten: true,
    };
    path.to_string()
}

fn sc_prompt(shell: &Shell, app: &ArgMatches) {
    let bold = format!("{}", Style(shell, &style::Bold, &""));
    let reset = format!("%{{{}{}%}}", color::Fg(color::Reset), style::Reset);
    // let fg = format!("%{{{}%}}", color::Fg(color::Rgb(0, 147, 255)));
    // let bg = format!("%{{{}%}}", color::Fg(color::Rgb(51, 232, 29)));
    let fg = format!("%{{{}%}}", color::Fg(color::Rgb(0, 192, 124)));
    let bg = format!("%{{{}%}}", color::Fg(color::Rgb(3, 97, 188)));

    let mut result: String = "".to_string();

    result.push_str(&bold);

    match app.value_of("lasterror") {
        Some(last_error) if last_error != "0" =>
            result.push_str(&format!("{}✘{}{:_>3} ", fg, bg, last_error)),
        _ =>
            result.push_str(&format!("{}✔{}__0 ", fg, bg)),
    }

    if let Some(hostname) = ssh_hostname() {
        result.push_str(&format!("{}⇆{}{} ", fg, bg, hostname));
    }

    if let Some(_) = virtualenv() {
        result.push_str(&format!("{}🐍\u{FE0E}{} ", fg, bg));
    }

    if let Ok(status) = Status::from_cwd() {
        result.push_str(&format!("{}🌲\u{FE0E}{}{} ", fg, bg, format(&status)));
    };

    if result.len() > 80 {
        result.push_str(&format!("{}\n{}", fg, bg));
    }

    result.push_str(&bg);
    result.push_str(match path() {Some(ref p) => &p, None => "!"});
    result.push_str(" ");
    result.push_str(&fg);
    result.push_str("∴ ");
    result.push_str(&reset);

    print!("{}", result);
}

fn sc_init(_: &ArgMatches) {
    println!(r#"
        PROMPT='$({exe} prompt --last-error $?)'
        function preexec {{ {exe} preexec "$1" }}
        function precmd {{ {exe} precmd }}
        typeset -a preexec_functions
        preexec_functions+=_make_prompt
        typeset -a precmd_functions
        precmd_functions+=_make_stop_prompt
        "#,
        exe=Path::new(&env::args().nth(0).unwrap()).canonicalize().unwrap().to_str().unwrap());
}

fn sc_preexec(app: &ArgMatches) {
    let mut cmd: String = app.value_of("command").unwrap().to_string();
    cmd = cmd.replace("&", ",");
    print!("\u{01b}]0;$ {}\u{007}", cmd);
}

fn sc_precmd(_: &ArgMatches) {
    let cmd = "";
    print!("\u{01b}]0;{}\u{007}", cmd);
}

fn main() {
    let app = App::new("prompt")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Prints your prompt.")
        .arg(Arg::with_name("shell")
                .takes_value(true)
                .long("shell")
                .default_value("zsh")
                .possible_values(&SupportedShells::variants())
                .case_insensitive(true))
        .subcommand(SubCommand::with_name("prompt")
            .arg(Arg::with_name("lasterror")
                .takes_value(true)
                .value_name("last error")
                .long("last-error")
                .help("-")))
        .subcommand(SubCommand::with_name("init")
            .about("Setup shell")
            .usage("Add this line to config. `eval \"$(prompt init)\"`"))
        .subcommand(SubCommand::with_name("preexec")
            .arg(Arg::with_name("command")
            .required(true)))
        .subcommand(SubCommand::with_name("precmd"))
        .get_matches();

    let shell: Box<Shell> = match value_t!(app, "shell", SupportedShells).unwrap() {
        SupportedShells::Zsh => Box::new(Zsh),
        SupportedShells::Bash => Box::new(Bash),
        SupportedShells::Fish => Box::new(Fish),
    };

    if let Some(matches) = app.subcommand_matches("prompt") {
        sc_prompt(&*shell, &matches);
    } else if let Some(matches) = app.subcommand_matches("init") {
        sc_init(&matches);
    } else if let Some(matches) = app.subcommand_matches("preexec") {
        sc_preexec(matches);
    } else if let Some(matches) = app.subcommand_matches("precmd") {
        sc_precmd(matches);
    } else {
        println!("{}", app.usage());
    }
}
