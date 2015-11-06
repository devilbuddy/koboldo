use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use std::collections::HashMap;
use std::string::String;

use sdl2::render::{Texture, Renderer};
use sdl2_image::LoadTexture;

// http://www.angelcode.com/products/bmfont/doc/file_format.html
const TAG_INFO : &'static str = "info";
const TAG_COMMON : &'static str = "common";
const TAG_PAGE : &'static str = "page";
const TAG_CHARS : &'static str = "chars";
const TAG_CHAR : &'static str = "char";

pub struct BitmapFont {
    texture : Texture,
    line_height : i32,
    glyphs : HashMap<char, Glyph>
}

pub struct BitmapFontBuilder {
    file_name : Option<String>,
    line_height : Option<i32>,
    glyphs : HashMap<char, Glyph>
}

impl BitmapFontBuilder {

    pub fn new() -> BitmapFontBuilder {
        BitmapFontBuilder {
            file_name : None,
            line_height : None,
            glyphs : HashMap::new()
        }
    }

    fn set_file_name(&mut self, file_name : String) {
        println!("set_file_name {:?}", file_name);
        self.file_name = Some(file_name);
    }

    fn set_line_height(&mut self, line_height : i32) {
        self.line_height = Some(line_height);
    }

    fn add_glyph(&mut self, c : char, glyph : Glyph) {
        self.glyphs.insert(c, glyph);
    }

    pub fn load(&mut self, font_file : &Path) {
        let reader = BufReader::new(File::open(&font_file).expect("Failed to load font file"));
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let pairs = line.split_whitespace().map(|key_value| {
                            let equals_index = key_value.find('=').unwrap_or(key_value.len());
                            let pair = key_value.split_at(equals_index);
                            (pair.0, pair.1.trim_matches('=').trim_matches('\"'))
                        }
                    ).collect::<HashMap<&str, &str>>();

                    if line.starts_with(TAG_INFO) {
                        for (&key, &value) in pairs.iter() {
                            println!("{}-{}", key, value);
                        }
                    } else if line.starts_with(TAG_COMMON) {
                        self.set_line_height(pairs.get("lineHeight").unwrap().parse::<i32>().unwrap());
                    } else if line.starts_with(TAG_PAGE) {
                        let file_name = pairs.get("file").unwrap();
                        let mut p = Path::new(font_file.to_str().unwrap());
                        self.set_file_name(p.parent().unwrap().join(file_name).to_str().unwrap().to_string());
                    } else if line.starts_with(TAG_CHARS) {

                    } else if line.starts_with(TAG_CHAR) {
                        let c = pairs.get("id").unwrap().parse::<u8>().unwrap() as char;

                        self.add_glyph(c, Glyph {
                            x : pairs.get("x").unwrap().parse::<i32>().unwrap(),
                            y : pairs.get("y").unwrap().parse::<i32>().unwrap(),
                            width : pairs.get("width").unwrap().parse::<i32>().unwrap(),
                            height : pairs.get("height").unwrap().parse::<i32>().unwrap(),
                            x_offset : pairs.get("xoffset").unwrap().parse::<i32>().unwrap(),
                            y_offset : pairs.get("yoffset").unwrap().parse::<i32>().unwrap(),
                            x_advance : pairs.get("xadvance").unwrap().parse::<i32>().unwrap()
                        });
                    }
                }
                _ => {}
            }

        }
    }

    pub fn build(self, renderer : &Renderer) -> Result<BitmapFont, &'static str> {
        //Err("Failed to build font")
        let s = self.file_name.unwrap();
        Ok(BitmapFont {
            texture : renderer.load_texture(Path::new(&s)).unwrap(),
            line_height : self.line_height.unwrap(),
            glyphs : self.glyphs
        })
    }


}

struct Glyph {
    x : i32,
    y : i32,
    width : i32,
    height : i32,
    x_offset : i32,
    y_offset : i32,
    x_advance : i32
}

impl BitmapFont {


}
