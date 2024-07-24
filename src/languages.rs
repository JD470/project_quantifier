use std::path::PathBuf;

pub enum Languages {
    None,
    Rust,
    Javascript,
    Python,
}

impl Languages {
    pub fn from(format: String) -> Self {
        match format.as_str() {
            ".rs" => Languages::Rust,
            ".js" | ".ts" => Languages::Javascript,
            ".py" => Languages::Python,
            _ => Languages::None,
        }
    }

    pub fn exclude_file(&self, path: PathBuf) -> bool {
        !match self {
            Languages::Rust => path.starts_with(".\\target"),
            Languages::Javascript => path.starts_with(".\\node_modules"),
            Languages::Python => path.starts_with(".\\env"),
            _ => true,
        }
    }
}
