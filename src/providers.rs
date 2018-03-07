use std::path::{Path, PathBuf};
use std::env;


pub struct PromptPath {
    pub home_string: Option<String>,
    pub seperator: String,
    pub shorten: bool
}


impl PromptPath {

    fn shorten_parts(parts: &mut Vec<String>) {
        let l = parts.len() - 1;
        for p in parts.iter_mut().take(l) {
            *p = p[..1].to_string();
        }
    }

    pub fn to_string(&self) -> Option<String> {
        let cwd = match Path::new(".").canonicalize() {
            Ok(path) => path.to_path_buf(),
            Err(_) => return None,
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

        let mut parts: Vec<String>;
        if let Some(hd) = home_path {
            parts = hd.components().map(|part|part.as_os_str().to_str().unwrap().to_string()).collect();
            parts.insert(0, self.home_string.clone().unwrap());
            if self.shorten {
                Self::shorten_parts(&mut parts);
            };
        } else {
            parts = cwd.components().map(|part|part.as_os_str().to_str().unwrap().to_string()).collect();
            if self.shorten {
                Self::shorten_parts(&mut parts);
            };
        }

        return Some(parts.join(&self.seperator).to_string());
    }
}
