#[derive(Debug)]
pub struct Formatter {
    parts: String,
}

impl Formatter {
    pub fn to_string() -> String {
        "".to_string()
    }

    pub fn add(mut self, part: &str) -> Self {
        self.parts.push_str(part);
        self
    }

    pub fn fg(mut self) -> Self {
        self
    }

    pub fn bg(mut self) -> Self {
        self
    }

    pub fn reset(mut self) -> Self {
        self
    }
}
