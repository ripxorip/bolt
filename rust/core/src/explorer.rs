use std::path;

pub struct Explorer {
    cwd: path::PathBuf,
}

impl Explorer {
    pub fn new(cwd: path::PathBuf) -> Explorer {
        Explorer{
            cwd: cwd,
        } 
    }

    pub fn get_cwd(&self) -> String {
        self.cwd.to_string_lossy().into_owned()
    }
}
