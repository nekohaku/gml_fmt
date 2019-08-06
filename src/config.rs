use std::path::PathBuf;
use std::{ffi::OsStr, fs};

pub struct Config {
    pub files: Vec<PathBuf>,
}

impl Config {
    pub fn new(input_path: PathBuf, do_file: bool) -> Result<Config, &'static str> {
        let mut config = Config { files: Vec::new() };

        if input_path.exists() == false {
            return Err("Filepath given does not exist.");
        }

        match (input_path.is_dir(), do_file) {
            (true, true) => {
                return Err("Passed -f or --file but gave a directory filepath.");
            }

            (true, false) => {
                fn take_in_gml_files(directory_path: &PathBuf, config: &mut Config) {
                    let gml_name = OsStr::new("gml");

                    for entry in fs::read_dir(directory_path)
                        .expect(&format!("Error reading directory {:?}.", directory_path))
                    {
                        let entry = entry.expect(&format!("Error reading file"));
                        let path = entry.path();

                        if path.is_dir() == false {
                            if path.extension() == Some(gml_name) {
                                config.load_file_path(path);
                            }
                        } else {
                            take_in_gml_files(&path, config);
                        }
                    }
                }

                take_in_gml_files(&input_path, &mut config);
            }

            (false, true) => {
                config.load_file_path(input_path);
            }

            (false, false) => {
                return Err("Did not pass -f but gave a filepath. Pass -f for files.");
            }
        };

        Ok(config)
    }

    pub fn load_file_path(&mut self, path: PathBuf) {
        self.files.push(path);
    }
}

pub enum PrintingFlags {
    NoOutput = 0b001,
    NormalOverwrite = 0b010,
    PrintLogs = 0b100,
}
