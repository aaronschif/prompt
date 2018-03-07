use std::path::{Path, PathBuf};
use std::env;


pub struct PromptPath {
    pub home_string: Option<String>,
    pub seperator: String,
    pub shorten: bool
}

pub struct PromptResult {
    home: bool,
    cwd: PathBuf,
}

impl PromptPath {

    fn shorten_parts(parts: &mut Vec<String>) {
        let l = parts.len() - 1;
        for p in parts.iter_mut().take(l).skip(1) {
            *p = p[..1].to_string();
        }
    }

    pub fn to_string(&self) -> String {
        let cwd = match Path::new(".").canonicalize() {
            Ok(path) => path.to_path_buf(),
            Err(_) => return "!".to_string(),
        };

        let home_path: Option<PathBuf> = {
            if self.home_string.is_some() {
                match env::var("HOME") {
                    Ok(homepath) => cwd.strip_prefix(&homepath).ok().map(|p| p.to_path_buf()),
                    Err(_)=> None,
                }
            } else {
                None
            }
        };

        let mut parts = cwd.components().map(|part|part.as_os_str().to_str().unwrap().to_string()).collect();

        if self.shorten {
            Self::shorten_parts(&mut parts);
        };

        if let Some(_) = home_path {
            parts[0] = self.home_string.clone().unwrap();
        } else {
            parts[0] = "".to_string();
        }

        return parts.join(&self.seperator).to_string();
    }
}
