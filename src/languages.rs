use std::path::PathBuf;

pub enum Languages {
    None,
    Markdown,
    Rust,
    Javascript,
    Typescript,
    Python,
    Ruby,
    Lua,
    Haskell,
    Bash,
    Batch,
    C,
    CHeader,
    CPP,
    CPPHeader,
}

impl Languages {
    pub fn from(format: &String) -> Self {
        match format.as_str() {
            ".md" => Languages::Markdown,
            ".rs" => Languages::Rust,
            ".js" => Languages::Javascript,
            ".ts" => Languages::Typescript,
            ".py" => Languages::Python,
            ".rb" => Languages::Ruby,
            ".lua" => Languages::Lua,
            ".hs" | ".lhs" => Languages::Haskell,
            ".sh" => Languages::Bash,
            ".bat" => Languages::Batch,
            ".c" | ".cc" => Languages::C,
            ".h" | ".hh" => Languages::CHeader,
            ".cpp" => Languages::CPP,
            ".hpp" => Languages::CPPHeader,
            _ => Languages::None,
        }
    }

    pub fn get_name(self) -> String {
        match self {
            Self::Markdown => "Markdown",
            Self::Rust => "Rust",
            Self::Javascript => "Javascript",
            Self::Typescript => "Typescript",
            Self::Python => "Python",
            Self::Ruby => "Ruby",
            Self::Lua => "Lua",
            Self::Haskell => "Haskell",
            Self::Bash => "Bash Script",
            Self::Batch => "Batch Script",
            Self::C => "C",
            Self::CHeader => "C Header",
            Self::CPP => "C++",
            Self::CPPHeader => "C++ Header",
            Self::None => "Other",
        }
        .to_string()
    }

    pub fn exclude_file(&self, path: PathBuf) -> bool {
        !match self {
            Languages::Rust => path.starts_with(".\\target"),
            Languages::Javascript | Languages::Typescript => path.starts_with(".\\node_modules"),
            Languages::Python => path.starts_with(".\\env"),
            _ => true,
        }
    }
}
