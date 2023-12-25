#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Go,
    Cpp,
    C,
    Csharp
}

impl Language {
    pub fn from_file_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Self::Rust),
            "go" => Some(Self::Go),
            "cpp" => Some(Self::Cpp),
            "c" => Some(Self::C),
            "cs" => Some(Self::Csharp),

            _ => None
        }
    }

    pub fn name(&self) -> String {
        let language = match self {
            Self::Rust => "Rust",
            Self::C => "C",
            Self::Csharp => "C#",
            Self::Cpp => "C++",
            Self::Go => "Go"
        };

        language.to_string()
    }
}