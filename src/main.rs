#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate clap;

mod inputimage;
mod outputmeta;
mod outputimage;
mod shapes;
mod packer;

use clap::{Arg, App};


fn main() {
	let matches = App::new("Atlasbuilder")
		.version("1.0.0")
		.author("Pete Ward <peteward44@gmail.com>")
		.about("Builds texture atlas images with JSON output")
		.arg(Arg::with_name("rotation-disable")
			.short("r")
			.long("rotation-disable")
			.help("Disable sub image rotation"))
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
		.arg(Arg::with_name("input")
			.help("Image filenames to add to atlas")
			.required(true)
			.multiple(true)
			.takes_value(true)
			.index(1))
		.get_matches();
	let inputs: Vec<&str> = matches.values_of("input").unwrap().collect();
//	println!( "{:?}", inputs );
	let output_width = matches.value_of("width").unwrap_or("2048").parse::<i32>().unwrap();
	let output_height = matches.value_of("height").unwrap_or("2048").parse::<i32>().unwrap();

	let mut output_meta = outputmeta::OutputMeta::new();
	let mut output = outputimage::OutputImage::new( output_width, output_height );
	let mut input = inputimage::InputImage::load( "test_images/input1_trim.png" );
	input.trim();
	
	let mut packer = packer::Packer::new( output_width, output_height );
	
	let pack_result: packer::PackResult = packer.pack( input.w, input.h, true ).unwrap();
	
	output.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	output_meta.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	
	let pack_result2 = packer.pack( input.w, input.h, true ).unwrap();
	
	println!( "Copying sub image {:?}", input.name );
	output.add_input( &input, pack_result2.rect.x, pack_result2.rect.y, pack_result2.rotated );
	output_meta.add_input( &input, pack_result2.rect.x, pack_result2.rect.y, pack_result2.rotated );
	
	println!( "Saving final output image {:?}", "out.png" );
	output.save( "test_images/out.png" );
	output_meta.save( "test_images/out.json", "hash", "out.png", output.w, output.h );
}

