extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate clap;
#[macro_use] extern crate failure;

mod inputimage;
mod outputmeta;
mod outputimage;
mod shapes;
mod packer;

use clap::{Arg, App};
use std::path::{ Path, PathBuf };
use std::fs;

fn is_image_file( p: &PathBuf ) -> bool {
	match p.extension() {
		Some( ext ) => {
			let lc = ext.to_str().unwrap_or( "" ).to_lowercase();
			lc == "jpg" || lc == "jpeg" || lc == "png"
		}
		None => false
	}
}

fn examine_dir( parent: &PathBuf, mut result: &mut Vec<PathBuf> ) -> Result<(), failure::Error> {
	for entry_ in fs::read_dir( parent )? {
		let p = entry_?.path();
		if p.is_file() {
			if is_image_file( &p ) {
				result.push( p );
			}
		} else if p.is_dir() {
			examine_dir( &p, &mut result )?;
		}
	}
	Ok(())
}

fn get_filenames( inputs: Vec<&str> ) -> Result<Vec<PathBuf>, failure::Error> {
	let mut result : Vec<PathBuf> = vec!();
	
	for input in inputs.iter() {
		let p = Path::new( input.clone() ).to_path_buf();
		if p.is_dir() { 
			examine_dir( &p, &mut result )?;
		} else {
			if is_image_file( &p ) {
				result.push( p );
			}
		}
	}
	
	Ok(result)
}

fn operate() -> std::result::Result<(), failure::Error> {
	let matches = App::new("Atlasbuilder")
		.version("1.0.0")
		.author("Pete Ward <peteward44@gmail.com>")
		.about("Builds texture atlas images with JSON output")
		.arg(Arg::with_name("rotation-disable")
			.short("r")
			.long("rotation-disable")
			.help("Disable sub image rotation"))
		.arg(Arg::with_name("fixed-size")
			.short("f")
			.long("fixed-size")
			.help("Output image will be a fixed width / height instead of attempting to use as little as possible"))
		.arg(Arg::with_name("width")
			.short("w")
			.long("width")
			.takes_value(true)
			.help("Maximum width of output atlas - must be power of 2"))
		.arg(Arg::with_name("height")
			.short("h")
			.long("height")
			.takes_value(true)
			.help("Maximum height of output atlas - must be power of 2"))
		.arg(Arg::with_name("output")
			.short("o")
			.long("output")
			.takes_value(true)
			.help("Output filename for .png file"))
		.arg(Arg::with_name("json")
			.short("j")
			.long("json")
			.takes_value(true)
			.help("Output filename for .json file"))
		.arg(Arg::with_name("input")
			.help("Image filenames to add to atlas")
			.required(true)
			.multiple(true)
			.takes_value(true)
			.index(1))
		.get_matches();
	let input_filenames: Vec<PathBuf> = get_filenames( matches.values_of("input").unwrap().collect() )?;
	println!( "{:?}", input_filenames );
	let output_width = matches.value_of("width").unwrap_or("2048").parse::<i32>().unwrap();
	let output_height = matches.value_of("height").unwrap_or("2048").parse::<i32>().unwrap();
	let output_filename = std::path::Path::new(matches.value_of("output").unwrap_or("out.png"));
	let output_json_filename = std::path::Path::new(matches.value_of("json").unwrap_or("out.json"));
	let allow_rotation = !matches.is_present("rotation-disable");
	let allow_grow = !matches.is_present("fixed-size");

	let mut packer = packer::Packer::new( output_width, output_height, allow_grow, allow_rotation );

	println!( "Calculating rects..." );
	let mut inputs: Vec<inputimage::InputImage> = vec!();
	for filename in input_filenames.iter() {
		let mut input = inputimage::InputImage::load( filename.to_str().unwrap() );
		input.trim();
		packer.add( input.vw, input.vh );
		inputs.push( input );
	}

	while !packer.pack() {
		if !packer.grow() {
			bail!( "Output size exceeded!" );
		}
	}

	let mut output_meta = outputmeta::OutputMeta::new();
	let mut output = outputimage::OutputImage::new( packer.get_w(), packer.get_h() );
	let pack_results = packer.get_results();
	for pack_result_index in 0..pack_results.len() {
		let pack_result: &packer::PackResult = &pack_results[pack_result_index];
		let input: &inputimage::InputImage = &inputs[pack_result_index];
		println!( "Copying sub image {:?}", input.name );
		output.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
		output_meta.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	}
	println!( "Outputting final image {:?}", output_filename );
	output.save( output_filename )?;
	output_meta.save( output_json_filename, "hash", output_filename.to_str().unwrap(), output.w, output.h )?;
	Ok(())
}


fn main() {
	let result = operate();
	match result {
		Err( e ) => println!("Error creating atlas {:?}", e ),
		Ok(_json) => println!("Complete!")
	}
}

