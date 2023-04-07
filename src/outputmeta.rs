use pathdiff::diff_paths;
use tera::Tera;
use super::shapes;
use super::inputimage;

#[derive(Serialize)]
struct JsonHashMeta {
	app: String,
	image: String,
	size: shapes::Size
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


#[derive(Serialize)]
struct SubImage {
	pub name: String,
	pub rotated: bool,
	pub trimmed: bool,
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

	pub fn add_input( &mut self, output_meta_root_dir: &std::path::Path, output_meta_filename_only: bool, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		let rect = SubImage{
			name: self.calculate_input_name(output_meta_root_dir, output_meta_filename_only, img.name.as_path()),
			rotated: rotated,
			trimmed: true,
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
		let meta = JsonHashMeta {
			app: "https://github.com/peteward44/atlasbuilder-rust".to_string(),
			image: self.calculate_output_name(output_meta_root_dir, output_meta_filename_only, image_output_path),
			size: shapes::Size {
				w: output_width,
				h: output_height
			}
		};
		
		let tera = Tera::new("templates/**/*.tmpl").unwrap();
		let mut context = tera::Context::new();
		context.insert("meta", &meta);
		context.insert("frames", &self.subs);

		let result = tera.render("json_hash.tmpl", &context)?;
		std::fs::write( filename, result.to_owned() )?;
		Ok(result)
	}
}
