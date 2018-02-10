use explorer::Explorer;
use explorer::BoltListEntry;
use std::vec;
use std::env;

pub struct Bolt {
    // Declare the explorers
    exp1: Explorer,
    exp2: Explorer,
}

impl Bolt {
    // Helper to get explorer with index
    fn get_exp<'a>(&'a self, exp_index: i32) -> &'a Explorer{
        if 1 == exp_index {
            &self.exp2
        }
        else{
            &self.exp1
        }
    }

    pub fn new() -> Bolt {
        let cwd = env::current_dir().unwrap();
        Bolt {
            exp1: Explorer::new(&cwd),
            exp2: Explorer::new(&cwd),
        }
    }

    pub fn get_cwd(&self,  exp_index:i32) -> String {
        self.get_exp(exp_index).get_cwd()
    }

    pub fn get_listing(&self, exp_index: i32) -> &vec::Vec<BoltListEntry> {
        self.get_exp(exp_index).get_listing()
    }

    pub fn cd(&mut self, exp_index: i32, id: i32) {
        if 1 == exp_index{
            self.exp2.cd(id);
        }
        else{
            self.exp1.cd(id);
        }
    }
}
