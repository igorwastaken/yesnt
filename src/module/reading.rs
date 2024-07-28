use std::path::PathBuf;
use std::fs;

pub struct Module {
    pub filename: PathBuf,
    pub script: String,
}

impl Module {
    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let script = fs::read_to_string(filename)?;
        Ok(Module {
            filename: PathBuf::from(filename),
            script,
        })
    }
}

