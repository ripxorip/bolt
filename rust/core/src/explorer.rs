use std::path;
use std::fs;
use std::vec;

pub struct Explorer {
    cwd: path::PathBuf,
    listing: vec::Vec<String>,
}

impl Explorer {
    fn create_listing(&mut self) {
        // Empty the current listing
        self.listing.clear();
        let paths = fs::read_dir(self.cwd.as_path()).unwrap();
        for path in paths {
            let p = path.unwrap().path();
            let filename = p.file_name().unwrap();
            self.listing.push(filename.to_string_lossy().into_owned());
        }
    }

    // Constructor
    pub fn new(cwd: &path::PathBuf) -> Explorer {
        let mut ret = Explorer{
            cwd: cwd.clone(),
            listing: vec::Vec::new(),
        }; 
        ret.create_listing();
        ret
    }

    // Returns a string representing the current working
    // directly
    pub fn get_cwd(&self) -> String {
        self.cwd.to_string_lossy().into_owned()
    }

    pub fn get_listing(&self) -> &vec::Vec<String> {
        &self.listing
    }

    pub fn cd(&mut self, id: i32){
        if id < 0 {
            self.cwd.pop();
        }
        self.create_listing();
    }
}
