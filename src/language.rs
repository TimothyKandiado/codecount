#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Go,
    Cpp,
    C,
    Csharp,
    Dart,
    Bash,
    Python,
    Java,
    GDScript,
    Julia,
    JavaScript,
    Kotlin,
    Lua,
    CHeaders,
    Nova,
}

impl Language {
    pub fn from_file_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Self::Rust),
            "go" => Some(Self::Go),
            "cpp" | "cc" | "cxx" => Some(Self::Cpp),
            "c" => Some(Self::C),
            "cs" => Some(Self::Csharp),
            "dart" => Some(Self::Dart),
            "sh" => Some(Self::Bash),
            "py" => Some(Self::Python),
            "java" => Some(Self::Java),
            "gd" => Some(Self::GDScript),
            "jl" => Some(Self::Julia),
            "js" => Some(Self::JavaScript),
            "kt" => Some(Self::Kotlin),
            "lua" => Some(Self::Lua),
            "h" => Some(Self::CHeaders),
            "nova" => Some(Self::Nova),

            _ => None,
        }
    }

    pub fn name(&self) -> String {
        let language = match self {
            Self::Rust => "Rust",
            Self::C => "C",
            Self::Csharp => "C#",
            Self::Cpp => "C++",
            Self::Go => "Go",
            Self::Dart => "Dart",
            Self::Bash => "Bash",
            Self::Python => "Python",
            Self::Java => "Java",
            Self::GDScript => "GDScript",
            Self::Julia => "Julia",
            Self::JavaScript => "JavaScript",
            Self::Kotlin => "Kotlin",
            Self::Lua => "Lua",
            Self::CHeaders => "C/C++ Headers",
            Self::Nova => "Nova",
        };

        language.to_string()
    }
}
