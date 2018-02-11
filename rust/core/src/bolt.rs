use explorer::Explorer;
use explorer::BoltListEntry;
use std::vec;
use std::env;
use std::slice;

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

    // Gets a list of indexes for the current listing in an explorer
    pub fn get_listing(&self, exp_index: i32, offset: usize, dest: &mut[i32]) -> i32 {
        let v = self.get_exp(exp_index).get_listing();
        let mut i = 0;
        while i < dest.len() && i + offset < v.len(){
            dest[i] = v[i].id;
            i += 1;
        }
        i as i32
    }

    // Returns the total number of entries in the filtered current dir
    pub fn get_num_entries(&self, exp_index: i32) -> i32 {
        self.get_exp(exp_index).get_num_entries()
    }

    // Change directory for exp_index to folder with id:id
    pub fn cd(&mut self, exp_index: i32, id: i32) {
        if 1 == exp_index{
            self.exp2.cd(id);
        }
        else{
            self.exp1.cd(id);
        }
    }
}
