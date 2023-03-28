extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;

mod inputimage;
mod outputmeta;
mod outputimage;
mod shapes;
mod packer;
mod parse_input_filenames;
mod outputdebug;

use clap::{Arg, Command};
use std::path::{ PathBuf };


fn operate() -> std::result::Result<(), failure::Error> {
	let matches = Command::new("atlasbuilder")
		//.version(crate_version!())
		.author("Pete Ward <peteward44@gmail.com>")
		.about("Builds texture atlas images with JSON output")
		.arg(Arg::new("rotation-disable")
			.short('r')
			.long("rotation-disable")
			.help("Disable sub image rotation"))
		.arg(Arg::new("fixed-size")
			.short('f')
			.long("fixed-size")
			.help("Output image will be a fixed width / height instead of attempting to use as little as possible"))
		.arg(Arg::new("width")
			.long("width")
			.num_args(1)
			.default_value("4096")
			.value_parser(clap::value_parser!(i32))
			.help("Maximum width of output atlas - must be power of 2"))
		.arg(Arg::new("height")
			.long("height")
			.num_args(1)
			.default_value("4096")
			.value_parser(clap::value_parser!(i32))
			.help("Maximum height of output atlas - must be power of 2"))
		.arg(Arg::new("output")
			.short('o')
			.long("output")
			.num_args(1)
			.default_value("out.png")
			.help("Output filename for .png file"))
		.arg(Arg::new("json")
			.short('j')
			.long("json")
			.num_args(1)
			.default_value("out.json")
			.help("Output filename for .json file"))
		.arg(Arg::new("padding")
			.short('p')
			.long("padding")
			.value_parser(clap::value_parser!(i32))
			.num_args(1)
			.default_value("2")
			.help("Pixel padding inbetween subimages"))
		.arg(Arg::new("input")
			.help("Image filenames to add to atlas")
			.required(true)
			.num_args(1..)
			.index(1))
		.get_matches();

	let raw_filenames = matches.get_many::<String>("input").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
	let input_filenames: Vec<PathBuf> = parse_input_filenames::parse(raw_filenames)?;
	let output_width = *matches.get_one::<i32>("width").expect("shouldn't happen");
	let output_height = *matches.get_one::<i32>("height").expect("shouldn't happen");
	let padding = *matches.get_one::<i32>("padding").expect("shouldn't happen");
	let output_filename = std::path::Path::new(matches.get_one::<String>("output").expect("shouldn't happen"));
	let output_json_filename = std::path::Path::new(matches.get_one::<String>("json").expect("shouldn't happen"));
	let allow_rotation = !matches.get_flag("rotation-disable");
	let allow_grow = !matches.get_flag("fixed-size");

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

