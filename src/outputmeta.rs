use serde::{Deserialize, Serialize};
use serde_json::{Result, to_string};
use std::collections::HashMap;
use std::fs;

use super::shapes;
use super::inputimage;

#[derive(Serialize, Deserialize)]
struct JsonHashFrame {
	rotated: bool,
	trimmed: bool,
	frame: shapes::Rect,
	spriteSourceSize: shapes::Rect,
	sourceSize: shapes::Size
}

#[derive(Serialize, Deserialize)]
struct JsonHashMeta {
	app: String,
	image: String,
	size: shapes::Size
}

#[derive(Serialize, Deserialize)]
struct JsonHash {
	frames: HashMap<String, JsonHashFrame>,
	meta: JsonHashMeta
}

#[derive(Serialize, Deserialize)]
struct JsonArrayFrame {
	filename: String,
	rotated: bool,
	trimmed: bool,
	frame: shapes::Rect,
	spriteSourceSize: shapes::Rect,
	sourceSize: shapes::Size
}

#[derive(Serialize, Deserialize)]
struct JsonArray {
	frames: Vec<JsonArrayFrame>,
	meta: JsonHashMeta
}

/*
// JSON HASH

{"frames": {

"image1":
{
    "frame": {"x":249,"y":205,"w":213,"h":159},
    "rotated": false,
    "trimmed": true,
    "spriteSourceSize": {"x":0,"y":0,"w":213,"h":159},
    "sourceSize": {"w":231,"h":175}
},
"image2":
{
    "frame": {"x":20,"y":472,"w":22,"h":21},
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": {"x":0,"y":0,"w":22,"h":21},
    "sourceSize": {"w":22,"h":21}
}},
"meta": {
    "app": "https://github.com/urraka/texpack",
    "image": "atlas.png",
    "size": {"w":650,"h":497}
    }
}

// JSON ARRAY

{"frames": [

{
    "filename": "image1",
    "frame": {"x":249,"y":205,"w":213,"h":159},
    "rotated": false,
    "trimmed": true,
    "spriteSourceSize": {"x":0,"y":0,"w":213,"h":159},
    "sourceSize": {"w":231,"h":175}
},
{
    "filename": "image2",
    "frame": {"x":29,"y":472,"w":22,"h":21},
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": {"x":0,"y":0,"w":22,"h":21},
    "sourceSize": {"w":22,"h":21}
}],
"meta": {
    "app": "https://github.com/urraka/texpack",
    "image": "atlas.png",
    "size": {"w":650,"h":497}
    }
}
*/



struct SubImage {
	pub name: String,
	pub rotated: bool,
	pub dest_x: i32,
	pub dest_y: i32,
	pub trimmed_x: i32,
	pub trimmed_y: i32,
	pub trimmed_w: i32,
	pub trimmed_h: i32,
	pub pretrimmed_w: i32,
	pub pretrimmed_h: i32
}

pub struct OutputMeta {
	subs: Vec<SubImage>
}

impl OutputMeta {
	pub fn new() -> OutputMeta {
		OutputMeta { subs: vec!() }
	}

	pub fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		let rect = SubImage{
			name: img.name.to_string(),
			rotated: rotated,
			dest_x: dx,
			dest_y: dy,
			trimmed_x: img.vx,
			trimmed_y: img.vy,
			trimmed_w: img.vw,
			trimmed_h: img.vh,
			pretrimmed_w: img.w,
			pretrimmed_h: img.h
		};
		self.subs.push( rect );
	}

	pub fn save( &self, filename: &std::path::Path, format: &str, output_name: &str, output_width: i32, output_height: i32 ) {
		let mut json;
		let meta = JsonHashMeta {
			app: "https://github.com/peteward44/atlasbuilder-rust".to_string(),
			image: output_name.to_string(),
			size: shapes::Size {
				w: output_width,
				h: output_height
			}
		};
		if format == "array" {
			let mut data: JsonArray = JsonArray{
				frames: Vec::new(),
				meta: meta
			};
			for sub in &self.subs {
				data.frames.push( JsonArrayFrame {
					filename: sub.name.to_string(),
					rotated: sub.rotated,
					trimmed: true,
					frame: shapes::Rect { x: sub.dest_x, y: sub.dest_y, w: sub.trimmed_w, h: sub.trimmed_h },
					spriteSourceSize: shapes::Rect { x: sub.trimmed_x, y: sub.trimmed_y, w: sub.trimmed_w, h: sub.trimmed_h },
					sourceSize: shapes::Size { w: sub.pretrimmed_w, h: sub.pretrimmed_h }
				} );
			}
			json = serde_json::to_string( &data );
		} else {
			let mut data: JsonHash = JsonHash{
				frames: HashMap::new(),
				meta: meta
			};
			for sub in &self.subs {
				data.frames.insert( sub.name.to_string(), JsonHashFrame {
					rotated: sub.rotated,
					trimmed: true,
					frame: shapes::Rect { x: sub.dest_x, y: sub.dest_y, w: sub.trimmed_w, h: sub.trimmed_h },
					spriteSourceSize: shapes::Rect { x: sub.trimmed_x, y: sub.trimmed_y, w: sub.trimmed_w, h: sub.trimmed_h },
					sourceSize: shapes::Size { w: sub.pretrimmed_w, h: sub.pretrimmed_h }
				} );
			}
			json = serde_json::to_string( &data );
		}
		
		if json.is_ok() {
			println!( "{:?}", json );
			std::fs::write( filename, json.unwrap() );
		}
	}
}
