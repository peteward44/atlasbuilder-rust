extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;

mod inputimage;
mod outputmeta;
mod outputimage;
mod shapes;
mod packer;
mod parse_input_filenames;
mod outputdebug;

use clap::{Arg, App};
use std::path::{ PathBuf };


fn operate() -> std::result::Result<(), failure::Error> {
	let matches = App::new("")
		.version(crate_version!())
		.version_short("v")
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
		.arg(Arg::with_name("padding")
			.short("p")
			.long("padding")
			.takes_value(true)
			.help("Pixel padding inbetween subimages"))
		.arg(Arg::with_name("input")
			.help("Image filenames to add to atlas")
			.required(true)
			.multiple(true)
			.takes_value(true)
			.index(1))
		.get_matches();

	let input_filenames: Vec<PathBuf> = parse_input_filenames::parse( matches.values_of("input").unwrap().collect() )?;
	let output_width = matches.value_of("width").unwrap_or("4096").parse::<i32>().unwrap();
	let output_height = matches.value_of("height").unwrap_or("4096").parse::<i32>().unwrap();
	let padding = matches.value_of("padding").unwrap_or("2").parse::<i32>().unwrap();
	let output_filename = std::path::Path::new(matches.value_of("output").unwrap_or("out.png"));
	let output_json_filename = std::path::Path::new(matches.value_of("json").unwrap_or("out.json"));
	let allow_rotation = !matches.is_present("rotation-disable");
	let allow_grow = !matches.is_present("fixed-size");

	let mut packer = packer::Packer::new( output_width, output_height, allow_grow, allow_rotation, padding );

	println!( "Calculating rects..." );
	let mut inputs: Vec<inputimage::InputImage> = vec!();
	for filename in input_filenames.iter() {
		let mut input = inputimage::InputImage::load( filename.to_str().unwrap() );
		input.trim();
		println!( "{{ w: {:?}, h: {:?} }}", input.vw, input.vh );
		inputs.push( input );
	}

	// sort by size then reverse order
	inputs.sort_by_key( |r| r.vw * r.vh );
	inputs.reverse();

	for input in inputs.iter() {
		packer.add( input.vw, input.vh );
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
		println!( "Copying sub image {:?} x={:?} y={:?} w={:?} h={:?}", input.name, pack_result.rect.x, pack_result.rect.y, pack_result.rect.w, pack_result.rect.h );
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

