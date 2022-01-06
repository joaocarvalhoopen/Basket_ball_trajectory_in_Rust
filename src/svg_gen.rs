/// File that creates and generates the SVG to a string or to a file.

// use std::io;
use std::fmt::Write;

pub enum Color {
    Black,
    White,
    Blue,
    Green,
    Red,
    Yellow,
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Black  => "black".to_string(),
            Color::White  => "white".to_string(),
            Color::Blue   => "blue".to_string(),
            Color::Green  => "green".to_string(),
            Color::Red    => "red".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
        }
    }
}

pub struct SVG {
    width: f32,
    height: f32,
    background_color: Option<Color>,
    elem_str_vec: Vec<String>,
}

impl SVG {
    pub fn new(width: f32, height: f32, background_color: Option<Color>) -> SVG {
        SVG {
            width,
            height,
            background_color,
            elem_str_vec: Vec::new(),
        }
    }

    pub fn add_elem(& mut self, elem_str: String) {
        self.elem_str_vec.push(elem_str);
    }

    // It doesn't make any intermediate allocation, only allocates one string buffer.
    pub fn to_string_append(&self, str_buf: & mut String) {
        if let Some(color) = & self.background_color {
            let _ = write!(str_buf, "<rect width=\"100%\" height=\"100%\" fill=\"{}\" />\n", color.to_string());
        } 

        // Problem: We need to test the boolean value so that the String.end_with("\n"), doesn't
        //          to be used. If it was used, because String's are UTF-8, the only possible way
        //          to interpret the size of each character at the end was if it scanned from the 
        //          beginning of the string to the end.
        // 
        //    TODO: Go deeper and find with details if this assumption are correct, because 
        //          in theory, the last 3 bytes of the four bytes string have a starting
        //          bit pattern that is different if they are from a single byte character,
        //          fom a second byte character, from a 3 bytes character, or from a 4 bytes
        //          character. With that in mind the function String.end_with("\n") would not
        //          need a full string scan from the beginning of the string, and it should be
        //          enough to do a scan of at most, the last 4 bytes of the string that
        //          internally would be accessed in a constant time by the underling Vec.
        //          That would mean that the cost of using it would be really small.
        //
        //    Conclusion: String.ends_width is not linear with the size of the string, it is constant
        //                time, so we can use it without fear of a full String scan from the beginning
        //                of the String.
        //                See this discussion about a similar method but on paths and it mentions
        //                UTF-8, char boundary detection, because each byte in a multi byte character
        //                has a preamble.
        //
        //              'Path::ends_with' is super super duper slow
        //              https://users.rust-lang.org/t/path-ends-with-is-super-super-duper-slow/18660
        //
        for elem_str in & self.elem_str_vec {
            str_buf.push_str(elem_str);
            if !elem_str.ends_with('\n') {
                str_buf.push('\n');
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut res_str = String::new();
        self.to_string_append(& mut res_str);

        res_str
    }

    // Calculate total capacity required for the string buffer, so it doesn't need to resize a make copies.
    fn calc_estimate_total_string_size(& self, preambule_len: Option<usize>) -> usize {
        let mut total_str_len = match preambule_len {
                                              Some(len) => len,
                                              // The maximum with is the to file preambule with background.
                                              None => 400,
                                      };
        for string_tmp in & self.elem_str_vec {
            total_str_len += string_tmp.len();
        }
        let newlines_coutner = self.elem_str_vec.len();
        total_str_len += newlines_coutner;

        total_str_len
    }

    /// It's faster, because it doesn't copy to intermediate memory the different substrings.
    ///
    /// To write to a byte stream like STDOUT, simply do a String.as_bytes() on the return value.
    ///    SVG.to_file_string().as_bytes()
    ///
    pub fn to_file_string(&self) -> String {
        // Calculate total capacity required for the string buffer, so it doesn't need to resize a make copies.
        let mut res_str = String::with_capacity(self.calc_estimate_total_string_size(None)); 

        // Write header.
        let _ = write!(res_str,
"<svg version=\"1.1\"
baseProfile=\"full\"
width=\"{0:.2}\" height=\"{1:.2}\"
xmlns=\"http://www.w3.org/2000/svg\"
xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n",
                self.width,
                self.height);

        // Write all body elements.
        self.to_string_append(& mut res_str);

        // Write footer.
        res_str.push_str("</svg>\n");
      
        res_str
    }

    /// Save to file.
    pub fn to_file(&self, filename: & str, file_path: & str) -> Result<(), String> {
        let res_str = self.to_file_string();

        use std::fs::File;
        use std::io::Write;

        let mut f;
        match File::create(String::new() + file_path + filename) {
            Ok(file) => f = file,
            Err(error) => {
                                    eprint!("{}", error);
                                    return Err(error.to_string());
                                }
        }
        match f.write_all(res_str.as_bytes()) {
            Ok(()) => (),
            Err(error) => {
                                    eprint!("{}", error);
                                    return Err(error.to_string());
                                }
        }
        Ok( () )
    }

    /// It's faster, because it doesn't copy to intermediate memory the different substrings.
    pub fn to_string_insert_in_html(&self) -> String {
        // Calculate total capacity required for the string buffer, so it doesn't need to resize a make copies.
        let mut res_str = String::with_capacity(self.calc_estimate_total_string_size(None)); 

        // Write header.
        let _= write!(res_str, 
                      "<svg width=\"{0:.2}\" height=\"{1:.2}\">\n",
                      self.width,
                      self.height);
        
        // Write all body elements.
        self.to_string_append(& mut res_str);

        // Write footer.
        res_str.push_str("</svg>\n");
      
        res_str
    }
    
}

