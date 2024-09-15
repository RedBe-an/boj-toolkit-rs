use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub struct FileManager {
    base_path: PathBuf,
}

impl FileManager {
    #[allow(dead_code)]
    pub fn new<P: Into<PathBuf>>(base_path: P) -> Self {
        FileManager {
            base_path: base_path.into(),
        }
    }

    #[allow(dead_code)]
    fn full_path(&self, file_name: &str) -> PathBuf {
        self.base_path.join(file_name)
    }

    #[allow(dead_code)]
    pub fn read(&self, file_name: &str) -> io::Result<String> {
        let mut file = File::open(self.full_path(file_name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[allow(dead_code)]
    pub fn write(&self, file_name: &str, contents: &str) -> io::Result<()> {
        File::create(self.full_path(file_name))?.write_all(contents.as_bytes())
    }

    #[allow(dead_code)]
    pub fn append(&self, file_name: &str, content: &str) -> io::Result<()> {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.full_path(file_name))?
            .write_all(content.as_bytes())
    }

    #[allow(dead_code)]
    pub fn exists(&self, file_name: &str) -> bool {
        self.full_path(file_name).exists()
    }

    #[allow(dead_code)]
    pub fn create_dir(&self, dir_name: &str) -> io::Result<()> {
        fs::create_dir_all(self.full_path(dir_name))
    }

    #[allow(dead_code)]
    pub fn delete(&self, file_name: &str) -> io::Result<()> {
        fs::remove_file(self.full_path(file_name))
    }

    #[allow(dead_code)]
    pub fn list_dir(&self, dir_name: &str) -> io::Result<Vec<String>> {
        fs::read_dir(self.full_path(dir_name))?
            .map(|entry| {
                entry.map(|e| e.file_name().into_string().unwrap_or_default())
            })
            .collect()
    }
}


#[allow(dead_code)]
pub fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

#[allow(dead_code)]
pub fn write_file(path: &Path, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

#[allow(dead_code)]
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}


