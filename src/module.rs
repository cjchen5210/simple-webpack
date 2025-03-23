use regex::Regex;

#[derive(Debug)]
pub struct Module {
    pub id: usize,
    pub path: String,
    pub code: String,
    pub dependencies: Vec<String>,    
}

impl Module {  
    pub fn new(id: usize, path: &str, code: &str) -> Self {
      let dependencies = Self::parse_imports(code);
        Module {
            id,
            path: path.to_string(),
            code: code.to_string(),
            dependencies: dependencies,
        }
    }

    pub fn parse_imports(code: &str) -> Vec<String> {
        let re = Regex::new(r#"import\s+.*?\s+from\s+['"](.+?)['"];"#).unwrap();
        re.captures_iter(code)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    pub fn from_file(id: usize, path: &str) -> Self {
        let code = std::fs::read_to_string(path).expect("Cannot read the file");
        Self::new(id, path, &code)
    }
}
