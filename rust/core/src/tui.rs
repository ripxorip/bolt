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
    pub fn draw(&self) -> String {
        // Leading char for header
        let leading = "#";
        // Temporary line
        let mut line = String::from("===============================================================");
        // Sting that will be returned
        let mut ret = "".to_string();
        self.get_header_line(leading, &line, &mut ret);
        line = String::from(" Bolt text user interface (alpha)");
        self.get_header_line(leading, &line, &mut ret);
        line = String::from("===============================================================");
        self.get_header_line(leading, &line, &mut ret);
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
