use explorer::Explorer;
use std::path;

pub struct Bolt {
    // Declare the explorers
    exp1: Explorer,
}

impl Bolt {
    pub fn new(cwd: path::PathBuf) -> Bolt {
        Bolt {
            exp1: Explorer::new(cwd),
        }
    }

    pub fn get_cwd_exp1(&self) -> String {
        self.exp1.get_cwd()
    }
}
