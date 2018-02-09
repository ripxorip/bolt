use explorer::Explorer;

pub struct Bolt {
    // Declare the explorers
    exp1: Explorer,
}

impl Bolt {
    pub fn new(cwd: String) -> Bolt {
        Bolt {
            exp1: Explorer::new(cwd),
        }
    }

    pub fn get_cwd_exp1(&self) -> &str {
        self.exp1.get_cwd()
    }
}
