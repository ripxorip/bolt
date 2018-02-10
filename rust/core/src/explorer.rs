use std::path;
use std::fs;
use std::vec;

pub struct BoltListEntry {
    pub id: i32,
    pub file_type: String,
    pub name: String,
}

pub struct Explorer {
    cwd: path::PathBuf,
    listing: vec::Vec<BoltListEntry>,
}

impl Explorer {
    fn get_entry_at_id(&self, id: i32) -> Option<&BoltListEntry>{
        let mut ret: Option<&BoltListEntry> = None;
        for entry in &self.listing {
            if id == entry.id {
                ret = Some(entry);
            }
        }
        ret
    }

    // Create listing for a new directory
    fn create_listing(&mut self) {
        // Empty the current listing
        self.listing.clear();
        let paths = fs::read_dir(self.cwd.as_path()).unwrap();
        let mut index = 0;
        for path in paths {
            let p = path.unwrap().path();
            let filename = p.file_name().unwrap();
            // Check for type, currently folder or not
            let file_type;
            if p.is_dir() {
                file_type = String::from("folder");
            }
            else{
                file_type = String::from("file");
            }
            let entry = BoltListEntry {
                id: index,
                file_type: file_type,
                name: filename.to_string_lossy().into_owned(),
            };
            self.listing.push(entry);
            index += 1;
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

    pub fn get_listing(&self) -> &vec::Vec<BoltListEntry> {
        &self.listing
    }

    pub fn cd(&mut self, id: i32){
        if id < 0 {
            self.cwd.pop();
        }
        else {
            // cd to folder with id
            let folder_name: String;
            {
                let entry_ref = self.get_entry_at_id(id).unwrap();
                folder_name = entry_ref.name.clone();
                println!("{}", folder_name);
            }
            self.cwd.push(folder_name);
        }
        self.create_listing();
    }
}
