pub struct Explorer {
    cwd: String,
}

impl Explorer {
    pub fn new(cwd: String) -> Explorer {
        Explorer{
            cwd: cwd,
        } 
    }

    pub fn get_cwd(&self) -> &str {
        &self.cwd
    }
}
