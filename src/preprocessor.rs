use std::io::{BufReader, Read, BufRead};

#[allow(dead_code)]
struct Preprocessor {
    cwd: String,
    inc_dirs: Vec<String>
}

#[allow(dead_code)]
impl Preprocessor {
    pub fn new(cwd: &str) -> Self {
        Self {
            cwd: String::from(cwd),
            inc_dirs: Vec::new()
        }
    }

    pub fn add_inc_dir(&mut self, dir: &str) {
        self.inc_dirs.push(String::from(dir));
    }

    pub fn resolve_path(&self, path: &str) -> Option<String> {
        if path.starts_with('/') {
            // absolute path
            Some(path.to_string())
        } else {
            // relative path
            // check if the path is in the cwd
            let mut full_path = self.cwd.clone();
            full_path.push('/');
            full_path.push_str(path);
            if std::path::Path::new(&full_path).exists() {
                return Some(full_path);
            }
            // check if the path is in the include directories
            for dir in &self.inc_dirs {
                let mut full_path = dir.clone();
                full_path.push('/');
                full_path.push_str(path);
                if std::path::Path::new(&full_path).exists() {
                    return Some(full_path);
                }
            }
            None
        }
    }

    pub fn preprocess(&self, top_src: &mut dyn Read) -> Result<String, String> {
        let mut res = String::new();
        let buffered_src = BufReader::new(top_src);

        for line in buffered_src.lines() {
            let line = line.map_err(|e| e.to_string())?;
            if line.starts_with("`include") {
                let path = line.split_whitespace().nth(1)
                    .ok_or("Missing path in `include directive")?;
                let path = path.trim_matches('"');
                let path = self.resolve_path(&path)
                    .ok_or(format!("Failed to resolve include path: {}", path))?;
                let mut inc_src = std::fs::File::open(&path)
                    .map_err(|e| format!("Failed to open include file: {}", e))?;
                let inc_content = self.preprocess(&mut inc_src)?;

                // replace the included content in the original source
                res.push_str(&inc_content);
                res.push('\n');
            }
            else {
                res.push_str(&line);
                res.push('\n');
            }
        }
        Ok(res)
    }
}