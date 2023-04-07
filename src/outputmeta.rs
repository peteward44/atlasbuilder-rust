use pathdiff::diff_paths;
use std::collections::HashMap;
use super::shapes;
use super::inputimage;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
	pub name: std::path::PathBuf,
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

	fn calculate_input_name( &self, output_meta_root_dir: &std::path::Path, output_meta_filename_only: bool, image_input_path: &std::path::Path ) -> String {
		if output_meta_filename_only {
			return image_input_path.file_name().unwrap().to_str().unwrap().to_string();
		}
		if output_meta_root_dir.eq(std::path::Path::new("")) {
			return image_input_path.to_str().expect("invalid path").to_owned();
		}
		return diff_paths(image_input_path, output_meta_root_dir).unwrap().to_str().unwrap().to_owned();
	}

	fn calculate_output_name( &self, output_meta_root_dir: &std::path::Path, output_meta_filename_only: bool, image_output_path: &std::path::Path ) -> String {
		return image_output_path.file_name().unwrap().to_str().unwrap().to_string();
	}

	pub fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		let rect = SubImage{
			name: img.name.to_owned(),
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

	pub fn save( &self, output_meta_root_dir: &std::path::Path, output_meta_filename_only: bool, filename: &std::path::Path, format: &str, image_output_path: &std::path::Path, output_width: i32, output_height: i32 ) -> std::result::Result<String, failure::Error> {
		let json;
		let meta = JsonHashMeta {
			app: "https://github.com/peteward44/atlasbuilder-rust".to_string(),
			image: self.calculate_output_name(output_meta_root_dir, output_meta_filename_only, image_output_path),
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
					filename: self.calculate_input_name(output_meta_root_dir, output_meta_filename_only, sub.name.as_path()),
					rotated: sub.rotated,
					trimmed: true,
					frame: shapes::Rect { x: sub.dest_x, y: sub.dest_y, w: sub.trimmed_w, h: sub.trimmed_h },
					spriteSourceSize: shapes::Rect { x: sub.trimmed_x, y: sub.trimmed_y, w: sub.trimmed_w, h: sub.trimmed_h },
					sourceSize: shapes::Size { w: sub.pretrimmed_w, h: sub.pretrimmed_h }
				} );
			}
			json = serde_json::to_string( &data )?;
		} else {
			let mut data: JsonHash = JsonHash{
				frames: HashMap::new(),
				meta: meta
			};
			for sub in &self.subs {
				data.frames.insert( self.calculate_input_name(output_meta_root_dir, output_meta_filename_only, sub.name.as_path()), JsonHashFrame {
					rotated: sub.rotated,
					trimmed: true,
					frame: shapes::Rect { x: sub.dest_x, y: sub.dest_y, w: sub.trimmed_w, h: sub.trimmed_h },
					spriteSourceSize: shapes::Rect { x: sub.trimmed_x, y: sub.trimmed_y, w: sub.trimmed_w, h: sub.trimmed_h },
					sourceSize: shapes::Size { w: sub.pretrimmed_w, h: sub.pretrimmed_h }
				} );
			}
			json = serde_json::to_string_pretty( &data )?;
		}
		
		std::fs::write( filename, json.to_string() )?;

		Ok(json)
	}
}
