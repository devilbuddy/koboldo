use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

use std::collections::HashMap;
use std::string::String;

use sdl2::rect::Rect;
use sdl2::render::{Texture, Renderer};
use sdl2_image::LoadTexture;

use sdl2::pixels::Color;

// http://www.angelcode.com/products/bmfont/doc/file_format.html
const TAG_INFO : &'static str = "info";
const TAG_COMMON : &'static str = "common";
const TAG_PAGE : &'static str = "page";
const TAG_CHARS : &'static str = "chars";
const TAG_CHAR : &'static str = "char";

pub struct BitmapFont {
    texture : Texture,
    pub line_height : i32,
    glyphs : HashMap<char, Glyph>
}

struct BitmapFontData {
    file_name : Option<String>,
    line_height : Option<i32>,
    glyphs : HashMap<char, Glyph>
}

impl BitmapFontData {
    fn new() -> BitmapFontData {
        BitmapFontData {
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
}

#[derive(Debug)]
struct Glyph {
    x : i32,
    y : i32,
    width : u32,
    height : u32,
    x_offset : i32,
    y_offset : i32,
    x_advance : i32
}

impl Glyph {
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl BitmapFont {

    pub fn draw_str(&self, s : &'static str, x : i32, y: i32, renderer : &mut Renderer) {
        let mut x_pos = x;
        let y_pos = y;
        for c in s.chars() {
            match self.glyphs.get(&c) {
                Some(glyph) => {
                    if !glyph.is_empty() {
                        renderer.copy(&self.texture,
                            Some(Rect::new_unwrap(glyph.x, glyph.y, glyph.width, glyph.height)),
                            Some(Rect::new_unwrap(x_pos + glyph.x_offset, y_pos + glyph.y_offset, glyph.width, glyph.height))
                        );
                    }
                    x_pos += glyph.x_advance;
                }
                _ => {}
            }
        }
    }


    pub fn load(font_file : &Path, renderer : &Renderer) -> Result<BitmapFont, &'static str> {
        let mut data = BitmapFontData::new();

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
                        /*
                        for (&key, &value) in pairs.iter() {
                            println!("{}-{}", key, value);
                        }
                        */
                    } else if line.starts_with(TAG_COMMON) {
                        data.set_line_height(pairs.get("lineHeight").unwrap().parse::<i32>().unwrap());
                    } else if line.starts_with(TAG_PAGE) {
                        let file_name = pairs.get("file").unwrap();
                        let p = Path::new(font_file.to_str().unwrap());
                        data.set_file_name(p.parent().unwrap().join(file_name).to_str().unwrap().to_string());
                    } else if line.starts_with(TAG_CHARS) {

                    } else if line.starts_with(TAG_CHAR) {
                        let c = pairs.get("id").unwrap().parse::<u8>().unwrap() as char;

                        data.add_glyph(c, Glyph {
                            x : pairs.get("x").unwrap().parse::<i32>().unwrap(),
                            y : pairs.get("y").unwrap().parse::<i32>().unwrap(),
                            width : pairs.get("width").unwrap().parse::<u32>().unwrap(),
                            height : pairs.get("height").unwrap().parse::<u32>().unwrap(),
                            x_offset : pairs.get("xoffset").unwrap().parse::<i32>().unwrap(),
                            y_offset : pairs.get("yoffset").unwrap().parse::<i32>().unwrap(),
                            x_advance : pairs.get("xadvance").unwrap().parse::<i32>().unwrap()
                        });
                    }
                }
                _ => {}
            }
        }

        // add glyph for space if missing
        if !data.glyphs.contains_key(&' ') {
            let space_width = data.glyphs.get(&'1').unwrap().x_advance;
            data.add_glyph(' ',
                Glyph {
                    x : 0,
                    y : 0,
                    width : 0,
                    height : 0,
                    x_offset : 0,
                    y_offset : 0,
                    x_advance : space_width
                }
            );
        }

        let s = data.file_name.unwrap();
        Ok(BitmapFont {
            texture : renderer.load_texture(Path::new(&s)).unwrap(),
            line_height : data.line_height.unwrap(),
            glyphs : data.glyphs
        })
    }

}
