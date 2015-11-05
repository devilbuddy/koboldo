use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use std::collections::HashMap;

// http://www.angelcode.com/products/bmfont/doc/file_format.html
const TAG_INFO : &'static str = "info";
const TAG_COMMON : &'static str = "common";
const TAG_PAGE : &'static str = "page";
const TAG_CHARS : &'static str = "chars";
const TAG_CHAR : &'static str = "char";

pub struct BitmapFont; /* {
}
*/

struct Glyph;

impl BitmapFont {

    pub fn load(font_file : &Path) -> Result<BitmapFont, &'static str> {

        let reader = BufReader::new(File::open(&font_file).expect("Failed to load font file"));
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let pairs = line.split_whitespace().map(|key_value| {
                            let equals_index = key_value.find('=').unwrap_or(key_value.len());
                            let pair = key_value.split_at(equals_index);
                            (pair.0, pair.1.trim_matches('='))
                        }
                    ).collect::<HashMap<&str, &str>>();

                    if line.starts_with(TAG_INFO) {
                        for (&key, &value) in pairs.iter() {
                            println!("{}-{}", key, value);
                        }
                    } else if line.starts_with(TAG_COMMON) {

                        for (&key, &value) in pairs.iter() {
                            println!("{}-{}", key, value);
                        }
                    } else if line.starts_with(TAG_PAGE) {
                        for (&key, &value) in pairs.iter() {
                            println!("{}-{}", key, value);
                        }
                    }
                }
                _ => {}
            }

        }

        Err("Failed to load font")
    }
}
