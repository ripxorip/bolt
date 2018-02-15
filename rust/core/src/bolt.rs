use explorer::Explorer;
use explorer::BoltListEntry;
use std::env;
use std::vec;

pub struct Bolt {
    // Declare the explorers
    exp1: Explorer,
    exp2: Explorer,
}

impl Bolt {
    // Local helper to get explorer with index
    fn get_exp<'a>(&'a self, exp_index: i32) -> &'a Explorer{
        if 1 == exp_index {
            &self.exp2
        }
        else{
            &self.exp1
        }
    }

    // Constructor
    pub fn new() -> Bolt {
        let cwd = env::current_dir().unwrap();
        Bolt {
            exp1: Explorer::new(&cwd),
            exp2: Explorer::new(&cwd),
        }
    }

    // Gets the current working directory for exp_id
    pub fn get_cwd(&self,  exp_index:i32) -> String {
        self.get_exp(exp_index).get_cwd()
    }

    // Get the name for an entry
    pub fn get_entry_name(&self, exp_index: i32, entry_id: i32) -> String {
        self.get_exp(exp_index).get_entry_name(entry_id)
    }

    // Get the name for an entry
    pub fn get_entry_type(&self, exp_index: i32, entry_id: i32) -> String {
        self.get_exp(exp_index).get_entry_type(entry_id)
    }

    // Raw listing, useful for e.g. the text GUI
    pub fn get_raw_listing(&self, exp_index: i32) -> &vec::Vec<BoltListEntry> {
        self.get_exp(exp_index).get_listing()
    }
    
    // Gets a list of indexes for the current listing in an explorer
    pub fn get_listing(&self, exp_index: i32, offset: usize, dest: &mut Vec<i32>) -> i32 {
        let v = self.get_exp(exp_index).get_listing();
        let mut i = 0;
        while i + offset < v.len(){
            dest.push(v[i].id);
            i += 1;
        }
        i as i32
    }

    // Returns the total number of entries in the filtered current dir
    pub fn get_num_entries(&self, exp_index: i32) -> i32 {
        self.get_exp(exp_index).get_num_entries()
    }

    // Change directory for exp_index to folder with id:id
    pub fn cd(&mut self, exp_index: i32, entry_id: i32) {
        if 1 == exp_index{
            self.exp2.cd(entry_id);
        }
        else{
            self.exp1.cd(entry_id);
        }
    }
}
