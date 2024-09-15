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
    fn full_path<T: AsRef<Path>>(&self, file_name: T) -> PathBuf {
        self.base_path.join(file_name)
    }

    #[allow(dead_code)]
    pub fn read<T: AsRef<Path>>(&self, file_name: T) -> io::Result<String> {
        let mut file = File::open(self.full_path(file_name))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[allow(dead_code)]
    pub fn write<T: AsRef<Path>, U: std::fmt::Display>(&self, file_name: T, contents: U) -> io::Result<()> {
        File::create(self.full_path(file_name))?.write_all(contents.to_string().as_bytes())
    }

    #[allow(dead_code)]
    pub fn append<T: AsRef<Path>, U: std::fmt::Display>(&self, file_name: T, content: U) -> io::Result<()> {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.full_path(file_name))?
            .write_all(content.to_string().as_bytes())
    }

    #[allow(dead_code)]
    pub fn exists<T: AsRef<Path>>(&self, file_name: T) -> bool {
        self.full_path(file_name).exists()
    }

    #[allow(dead_code)]
    pub fn create_dir<T: AsRef<Path>>(&self, dir_name: T) -> io::Result<()> {
        fs::create_dir_all(self.full_path(dir_name))
    }

    #[allow(dead_code)]
    pub fn delete<T: AsRef<Path>>(&self, file_name: T) -> io::Result<()> {
        fs::remove_file(self.full_path(file_name))
    }

    #[allow(dead_code)]
    pub fn list_dir<T: AsRef<Path>>(&self, dir_name: T) -> io::Result<Vec<String>> {
        fs::read_dir(self.full_path(dir_name))?
            .map(|entry| {
                entry.map(|e| e.file_name().into_string().unwrap_or_default())
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn move_dir(&mut self, new_base_path: &str) -> io::Result<()> {
        let new_path = PathBuf::from(new_base_path);
        
        if !new_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "새 경로가 존재하지 않습니다."));
        }
        
        if !new_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "새 경로가 디렉토리가 아닙니다."));
        }
        
        // 현재 base_path의 내용을 새 경로로 이동
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            let new_dest = new_path.join(path.file_name().unwrap());
            fs::rename(path, new_dest)?;
        }
        // base_path 업데이트
        self.base_path = new_path;
        
        Ok(())
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


