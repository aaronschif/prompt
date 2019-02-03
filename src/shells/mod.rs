use termion::color::Color;
use std::fmt;

pub struct Bash;
pub struct Zsh;
pub struct Fish;

pub trait Shell {
    fn escape(&self, f: &mut fmt::Formatter, thing: &fmt::Display, thing2: &fmt::Display);
}

impl Shell for Bash {
    fn escape(&self, f: &mut fmt::Formatter, thing: &fmt::Display, thing2: &fmt::Display) {
        write!(f, "\033[31m{}{}\033[m", thing, thing2);
    }
}

impl Shell for Zsh {
    fn escape(&self, f: &mut fmt::Formatter, thing: &fmt::Display, thing2: &fmt::Display) {
        write!(f, "%{{{}{}%}}", thing, thing2);
    }
}

impl Shell for Fish {
    fn escape(&self, f: &mut fmt::Formatter, thing: &fmt::Display, thing2: &fmt::Display) {
        write!(f, "{}{}", thing, thing2);
    }
}

pub struct Style<'a> (pub &'a Shell,pub &'a fmt::Display,pub &'a fmt::Display);

impl<'a> fmt::Display for Style<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.escape(f, self.1, self.2);
        Result::Ok(())
    }
}

// pub enum Shells {
//     Bash,
//     Zsh,
//     Fish,
// }
//
// impl Shell for Shells {
//     fn escape(&self, f: &mut fmt::Formatter, thing: &fmt::Display) {
//         match self {
//             Shells::Zsh => {write!(f, "%{{{{{}%}}}}", thing);},
//             Shells::Bash => {write!(f, "\033[31m{}\033[m", thing);},
//             Shells::Fish => {write!(f, "{}", thing);},
//         };
//     }
// }

// impl Shells {
//     fn escape(&self, color: &impl Color) -> String {
//         match self {
//
//         }
//     }
// }


// ZSH %{{ %}}
// BASH \033[31m \033[m
