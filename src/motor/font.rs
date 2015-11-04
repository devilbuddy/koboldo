use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

// http://www.angelcode.com/products/bmfont/doc/file_format.html

const TAG_INFO : &'static str = "info";
const TAG_COMMON : &'static str = "common";
const TAG_PAGE : &'static str = "page";
const TAG_CHARS : &'static str = "chars";
const TAG_CHAR : &'static str = "char";

pub struct BitmapFont; /* {
    texture : sdl2::render::Texture
}
*/
impl BitmapFont {
    pub fn load(font_file : &Path)  {
        let mut reader = BufReader::new(File::open(&font_file).expect("Failed to load font file"));

        let line = &mut String::new();
        let mut done = false;

        while !done {
            match reader.read_line(line) {
                Ok(size) => {
                    done = size == 0;

                    //println!("{:?}", line);
                    if line.starts_with(TAG_INFO) {
                        //println!("info");
                    } else if line.starts_with(TAG_CHAR) {

                    }


                }
                _ => { done = true; }
            }
        }


    }
}
