/* Class for the text GUI of bolt */
use bolt::Bolt;

pub struct Tui {
    width: u32,
    height: u32,
    bolt: Bolt,
}

impl Tui {
    // Constructor
    pub fn new(width: u32, height: u32) -> Tui {
        Tui {
            width: width,
            height: height,
            bolt: Bolt::new(),
        }
    }

    //================================================
    // Key Handlers
    //================================================
    // Selection up
    // Selection down
    // Tab
    // Enter
    // Update filter

    //================================================
    // Drawing
    //================================================
    // Draw - returns a string containing the GUI
    // simplified version for just one explorer
    pub fn draw(&self) -> String {
        // Start of header
        // Leading char for header
        let leading = "#";
        // Temporary line
        let mut line = String::from("===============================================================");
        // Sting that will be returned
        let mut ret = "".to_string();
        self.get_header_line(leading, &line, &mut ret);
        line = String::from(" Bolt text user interface (alpha)");
        self.get_header_line(leading, &line, &mut ret);
        // Get path
        let cwd = " $>".to_string() + &self.bolt.get_cwd(0);
        self.get_header_line(leading, &cwd, &mut ret);
        line = String::from("===============================================================");
        self.get_header_line(leading, &line, &mut ret);
        let mut listing: Vec<i32> = Vec::new();
        self.bolt.get_listing(0, 0, &mut listing);
        // End of header
        for index in listing {            
            let t = self.bolt.get_entry_type(0, index);
            if(t == "folder") {
                ret.push_str("+");
            }
            ret.push_str(&self.bolt.get_entry_name(0,index));
            ret.push_str("\n");
        }
        ret
    }

    //================================================
    // Helpers
    //================================================
    pub fn get_header_line(&self, leading: &str, input: &String, dest: &mut String) {
        dest.push_str(leading);
        dest.push_str(input);
        dest.push_str("\n");
    }
}
