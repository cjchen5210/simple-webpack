mod module;
use module::Module;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct Bundler {
    entry: String,
    modules: HashMap<String, Module>,
    next_id: usize,
}

impl Bundler {
    fn new(entry: &str) -> Self {
        Bundler {
            entry: entry.to_string(),
            modules: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn build(&mut self) {
        let entry = self.entry.clone();
        self.analyze_module(&entry);
    }

    fn analyze_module(&mut self, path: &str) {
        if self.modules.contains_key((path)) {
            return;
        }

        let module = Module::from_file((self.next_id), path);
        self.next_id += 1;
        let dependencies = module.dependencies.clone();
        self.modules.insert(path.to_string(), module);

        for dep in dependencies {
            let abs_path = Path::new(path)
                .parent()
                .unwrap()
                .join(&dep)
                .to_str()
                .unwrap()
                .to_string();
            self.analyze_module(&abs_path);
        }
    }

    pub fn generate_bundle(&self) -> String {
        let mut output = String::new();
        output.push_str("(function(modules) {\n");
        output.push_str("  function require(moduleId) {\n");
        output.push_str("    var module = { exports: {} };\n");
        output.push_str("    modules[moduleId](module, module.exports, require);\n");
        output.push_str("    return module.exports;\n");
        output.push_str("  }\n");
        output.push_str("  require(0);\n");
        output.push_str("})({\n");

        for (idx, (path, module)) in self.modules.iter().enumerate() {
            output.push_str(&format!(
                "  {}: function(module, exports, require) {{\n",
                idx
            ));
            output.push_str(&format!("    {}\n", module.code));
            output.push_str("  },\n");
        }
        output.push_str("});\n");

        output
    }
}

fn main() {
    let mut bundler = Bundler::new("./src/example/index.js");
    bundler.build();
    let bundle = bundler.generate_bundle();
    fs::create_dir_all("./dist").expect("cannot create dist");
    fs::write("./dist/bundle.js", &bundle).expect("cannot write to the file");
    println!("done! ->  ./dist/bundle.js");
}
