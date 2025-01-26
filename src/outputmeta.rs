use pathdiff::diff_paths;
use std::path::Path;
use std::env;
use tera::Tera;
use super::shapes;
use super::inputimage;

// "https://github.com/urraka/texpack"

#[derive(Serialize)]
struct JsonHashMeta {
	pub app: String,
	pub path_absolute: String,
	pub path_relative: String,
	pub filename: String,
	pub basename: String,
	pub extension: String,
	pub size: shapes::Size,
}

#[derive(Serialize)]
struct SubImage {
	pub path_absolute: String,
	pub path_relative: String,
	pub filename: String,
	pub basename: String,
	pub extension: String,
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
	subs: Vec<SubImage>,
	tera: Option<Tera>,
}

fn get_templates_directory() -> String {
    let mut dir = env::current_exe().expect("Could not get templates directory");
    dir.pop();
    dir.push("templates");
    dir.into_os_string().into_string().unwrap()
}

impl OutputMeta {
	pub fn new() -> OutputMeta {
		let templates_directory = get_templates_directory();
		let tera: Option<Tera>;
		if Path::new(&templates_directory).exists() {
			tera = Some(Tera::new((templates_directory + "/**/*").as_str()).unwrap());
		} else {
			tera = None;
		}
		OutputMeta {
			subs: vec!(),
			tera,
		}
	}

	fn calculate_absolute_path( &self, image_input_path: &std::path::Path ) -> String {
		return std::fs::canonicalize(&image_input_path).unwrap().to_str().unwrap().to_owned();
	}

	fn calculate_relative_path( &self, output_meta_root_dir: &std::path::Path, image_input_path: &std::path::Path ) -> String {
		if output_meta_root_dir.eq(std::path::Path::new("")) {
			return image_input_path.to_str().expect("invalid path").to_owned();
		}
		return diff_paths(image_input_path, output_meta_root_dir).unwrap().to_str().unwrap().to_owned();
	}

	fn calculate_filename( &self, image_input_path: &std::path::Path ) -> String {
		return image_input_path.file_name().unwrap().to_str().unwrap().to_owned();
	}

	fn calculate_basename( &self, image_input_path: &std::path::Path ) -> String {
		return image_input_path.file_stem().unwrap().to_str().unwrap().to_owned();
	}

	fn calculate_extension( &self, image_input_path: &std::path::Path ) -> String {
		return image_input_path.extension().unwrap().to_str().unwrap().to_owned();
	}

	pub fn add_input( &mut self, input_name_root_dir: &std::path::Path, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		let rect = SubImage{
			path_absolute: self.calculate_absolute_path(img.name.as_path()),
			path_relative: self.calculate_relative_path(input_name_root_dir, img.name.as_path()),
			filename: self.calculate_filename(img.name.as_path()),
			basename: self.calculate_basename(img.name.as_path()),
			extension: self.calculate_extension(img.name.as_path()),
			rotated,
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

	pub fn save( &self, filename: &std::path::PathBuf, template: &str, output_name_root_dir: &std::path::Path, image_output_path: &std::path::Path, output_width: i32, output_height: i32 ) -> std::result::Result<String, failure::Error> {
		match self.tera {
			None => {
				bail!("No valid templates configured")
			},
			_ => {},
		}
		let meta = JsonHashMeta {
			app: "https://github.com/peteward44/atlasbuilder-rust".to_string(),
			path_absolute: self.calculate_absolute_path(image_output_path),
			path_relative: self.calculate_relative_path(output_name_root_dir, image_output_path),
			filename: self.calculate_filename(image_output_path),
			basename: self.calculate_basename(image_output_path),
			extension: self.calculate_extension(image_output_path),
			size: shapes::Size {
				w: output_width,
				h: output_height
			}
		};
		
		let mut context = tera::Context::new();
		context.insert("meta", &meta);
		context.insert("frames", &self.subs);

		// test if template is one of the predefined ones, or if the user has specified a filename
		let result = match Path::new(template).try_exists() {
			Ok(true) => {
				let string = std::fs::read_to_string(template)?;
				Tera::one_off(string.as_str(), &context, false)?
			},
			_ => self.tera.as_ref().unwrap().render(template, &context)?,
		};
		
		let parent_dir = filename.parent();
		if parent_dir.is_some()
		{
			std::fs::create_dir_all(parent_dir.unwrap())?;
		}
		std::fs::write(filename, result.to_owned())?;
		Ok(result)
	}
}
