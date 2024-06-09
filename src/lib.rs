use std::fs::{self, File};


///Represents the configuration wrote in the Neo file
#[derive(Debug)]
struct NeoConfig {
    name: String,
    language: String,
    version: String,
    files_to_compile: Vec<String>,
}

///Process the Neo file and returns a NeoConfig struct
/// ### Arguments
/// * `data` - The content of the Neo file
/// ### Returns
/// * `NeoConfig` - The configuration of the Neo file
pub fn process_file(data: String) -> Option<NeoConfig> {
    let mut name = String::new();
    let mut language = String::new();
    let mut version = String::new();
    let mut files_to_compile = Vec::new();

    for line in data.lines() {
        let mut parts = line.split(":");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        match key {
            "name" => name = value.to_string(),
            "language" => language = value.trim().to_string(),
            "version" => version = value.trim().to_string(),
            "sources" => {
                if let Ok(files) = fs::read_dir(value.trim()) {
                    files_to_compile = files
                        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
                        .collect();
                } else {
                    return None;
                }
            },
            _ => {}
        }
    }

    return Some(NeoConfig {
        name,
        language,
        version,
        files_to_compile,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = fs::read_to_string("./main.neo").unwrap();
        let config = process_file(data).unwrap();
        println!("{:?}", config);
    }
}