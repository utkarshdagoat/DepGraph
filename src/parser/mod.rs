use crate::graph::Graph;
use std::char;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
mod python_parser;
use crate::parser::python_parser::clean_import;

#[allow(dead_code)]
pub struct ParserStruct {
    /// Contians the delimeter to split the import statements by
    /// Keywords to look for in the import statement
    delim: Option<char>,
    keywords: Vec<(String, usize)>,
    depgraph: Graph<String>,
    extension: String,
}
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
#[allow(dead_code)]
impl ParserStruct {
    pub fn new(extension: String) -> Self {
        Self {
            delim: None,
            keywords: Vec::new(),
            depgraph: Graph::new(),
            extension,
        }
    }

    pub fn find_keyword(&mut self, file: PathBuf) {
        let lines = fs::read_to_string(file).expect("Cannot Find Parser File");
        for line in lines.lines() {
            let words = line.split_whitespace();
            for (i, word) in words.clone().enumerate() {
                if word.contains("[name]") {
                    self.keywords.push((
                        words
                            .clone()
                            .nth(i - 1)
                            .expect("parser error found [name] as the first argument")
                            .to_string(),
                        i,
                    ));
                    // word contains some sort of padding around it
                    if word != "[name]" {
                        self.find_delim(word);
                    }
                }
            }
        }
    }

    pub fn find_delim(&mut self, word: &str) {
        // This method is called when the [name] locator is wrapped in some sort of extra padding
        // Example:
        //  For a rust crate the import might look like
        //  use crate::[name]
        //  Thus here we have a delim in form of :
        let chars: Vec<char> = word.chars().collect();
        let mut chars_iter = chars.clone().into_iter();
        for (i, char) in chars_iter.clone().enumerate() {
            if char == '[' && chars[i + 5] == ']' {
                self.delim = chars_iter.nth(i - 1);
            }
        }
    }

    pub fn extract_files(&mut self, dir: &PathBuf) -> Result<(), io::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.extract_files(&path)?;
                } else if entry.path().extension().is_some()
                    && entry.path().extension().unwrap() == self.extension.as_str()
                {
                    self.handle_file(&entry, dir);
                }
            }
        }
        Ok(())
    }

    pub fn handle_file(&mut self, file: &DirEntry, dir: &Path) {
        let cleaned_file = clean_dir(&file.path())
            .trim_end_matches(format!(".{}/", self.extension).as_str())
            .to_string();
        let contents = fs::read_to_string(file.path()).expect("Cannot Read File for imports");
        let mut vec_imports: Vec<_> = Vec::new();
        for line in contents.lines() {
            for keyword in self.keywords.iter() {
                if line.starts_with(keyword.0.as_str()) {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    let import = clean_import(words[keyword.1], dir);
                    vec_imports.push(import);
                }
            }
        }
        self.push_import_in_graph(cleaned_file, vec_imports);
        println!("\n");
    }

    pub fn push_import_in_graph(&mut self, file: String, imports: Vec<String>) {
        for import in imports {
            let file_hash = calculate_hash(&file);
            let import_hash = calculate_hash(&import);
            self.depgraph.push_vertice(file_hash, file.to_string());
            if self.depgraph.vertices.get(&import_hash).is_none() {
                self.depgraph.push_vertice(import_hash, import);
            }
            self.depgraph.push_edge(file_hash, import_hash);
        }
    }
}

pub fn clean_dir(dir: &Path) -> String {
    let cleaned_dir = dir.to_str().unwrap().replace("./projects/", "");
    format!("{}{}", cleaned_dir, "/")
}

#[test]
fn keyword_test() {
    let mut parser = ParserStruct::new("py".to_string());
    parser.find_keyword(PathBuf::from("./src/parser/example/py.parser"));
    println!("{:?}", parser.keywords);
    println!("{:?}", parser.delim);
    parser
        .extract_files(&PathBuf::from("./projects"))
        .expect("Some Error Occured");
}
