mod language;

use std::{fs::{self, ReadDir}, path::PathBuf, sync::{RwLock, OnceLock}, io::{BufReader, BufRead}};
use std::collections::HashMap;

use language::Language;

static FILES: OnceLock<RwLock<HashMap<String, Vec<PathBuf>>>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
struct LanguageLines {
    pub name: String,
    pub lines: usize
}

impl PartialOrd for LanguageLines {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LanguageLines {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lines.cmp(&other.lines)
    }
}

pub fn start(path: PathBuf) {
    FILES.set(RwLock::new(HashMap::new())).unwrap();

    let result = fs::read_dir(path);

    if let Err(err) = result {
        println!("{}", err);
        return;
    }

    let dir = result.unwrap();
    find_files(dir);
    
    #[cfg(feature = "debug")]
    for (language, filelist) in &*FILES.get().unwrap().read().unwrap() {
        print!("{language}: ");
        for file in filelist {
            print!("{:#?}", file.file_name().unwrap())
        }
        println!();
    }

    let mut language_lines = start_count();
    language_lines.sort();
    language_lines.reverse();

    let mut total_lines = 0usize;

    println!("================= Lines of code ==================");
    for code_lines in language_lines {
        println!("{:25} : {:>22}", code_lines.name, code_lines.lines);
        total_lines += code_lines.lines;
    }
    println!("==================================================");
    println!("{:25} : {:>22}", "Total",  total_lines);
    println!("==================================================");
}

fn find_files(dir: ReadDir) {
    for entry in dir {
        if let Err(err) = entry {
            println!("{}", err);
            continue;
        }

        let entry = entry.unwrap();
        
        let filetype = entry.file_type();
        if let Err(err) = filetype{
            println!("{}", err);
            continue;
        }

        let filetype = filetype.unwrap();
        
        if filetype.is_file() {
            let file_path = entry.path();
            add_file(file_path);
        }
        else if filetype.is_dir(){
            let dir = entry.path().read_dir();
            if let Err(err) = dir {
                println!("{}", err);
                continue;
            }

            let dir = dir.unwrap();
            find_files(dir)
        }
    }
}

fn add_file(file_path: PathBuf) {
    let extension = file_path.extension();
    if extension.is_none() {
        return
    }

    let extension = extension.unwrap();
    let extension = extension.to_str().unwrap_or("");

    let language = Language::from_file_extension(extension);

    if let Some(language) = language {
        let binding = &mut (*FILES.get().unwrap().write().unwrap());

        if let Some(file_list) = binding.get_mut(&language.name()) {
            file_list.push(file_path);
            return;
        }

        binding.insert(language.name(), vec![file_path]);
    }
}

fn start_count() -> Vec<LanguageLines> {
    let binding = FILES.get().unwrap().read().unwrap();
    let mut code_lines = Vec::new();
    
    for (language, filelist) in &*binding {
        let mut lines = 0usize;
        for file in filelist {
            lines += count_lines_in_file(file);
        }
        
        code_lines.push(LanguageLines {name: language.clone(), lines})
    }

    code_lines
}

fn count_lines_in_file(path: &PathBuf) -> usize {
    let result = fs::File::open(path);
    if let Err(err) = result {
        println!("{}", err);
        return 0;
    }

    let file = result.unwrap();
    let mut buffered_reader = BufReader::new(file);
    let mut line = String::new();
    let mut num_lines = 0usize;

    while let Ok(bytes) = buffered_reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }

        if bytes > 1 {
            num_lines += 1;
        }

        line.clear();
    }

    num_lines
}