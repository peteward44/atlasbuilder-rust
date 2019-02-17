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
			.long("rotation")
			.help("Disable sub image rotation"))
		.arg(Arg::with_name("input")
			.help("Sets the input file to use")
			.required(true)
			.multiple(true)
			.takes_value(true)
			.index(1))
		.get_matches();
	let inputs: Vec<&str> = matches.values_of("input").unwrap().collect();
	println!( "{:?}", inputs );

	let mut output_meta = outputmeta::OutputMeta::new();
	let mut output = outputimage::OutputImage::new( 600, 600 );
	let mut input = inputimage::InputImage::load( "test_images/input1_trim.png" );
	input.trim();
	
	let mut packer = packer::Packer::new( 2048, 2048 );
	
	let pack_result = packer.pack( input.w, input.h, true );
	
	output.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	output_meta.add_input( &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	
	let pack_result2 = packer.pack( input.w, input.h, true );
	
	output.add_input( &input, pack_result2.rect.x, pack_result2.rect.y, pack_result2.rotated );
	output_meta.add_input( &input, pack_result2.rect.x, pack_result2.rect.y, pack_result2.rotated );
	
	output.save( "test_images/out.png" );
	output_meta.save( "test_images/out.json", "hash" );
}

