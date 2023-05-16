extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;
#[macro_use] extern crate clap;

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
		.author("Pete Ward <peteward44@gmail.com>")
		.version(crate_version!())
		.about("Builds texture atlas images with meta data output")
		.arg(Arg::new("rotation-disable")
			.short('r')
			.long("rotation-disable")
			.action(clap::ArgAction::SetTrue)
			.help("Disable sub image rotation"))
		.arg(Arg::new("fixed-size")
			.short('f')
			.long("fixed-size")
			.action(clap::ArgAction::SetTrue)
			.help("Output image will be a fixed width / height instead of attempting to use as little as possible"))
		.arg(Arg::new("width")
			.long("width")
			.action(clap::ArgAction::Set)
			.default_value("4096")
			.value_parser(clap::value_parser!(i32))
			.help("Maximum width of output atlas - must be power of 2"))
		.arg(Arg::new("height")
			.long("height")
			.action(clap::ArgAction::Set)
			.default_value("4096")
			.value_parser(clap::value_parser!(i32))
			.help("Maximum height of output atlas - must be power of 2"))
		.arg(Arg::new("image-output")
			.short('o')
			.long("image-output")
			.action(clap::ArgAction::Set)
			.default_value("out.png")
			.help("Output filename for .png file"))
		.arg(Arg::new("meta-output")
			.long("meta-output")
			.action(clap::ArgAction::Set)
			.default_value("")
			.help("Output filename for meta file"))
		.arg(Arg::new("meta-template")
			.short('m')
			.long("meta-template")
			.action(clap::ArgAction::Set)
			.default_value("json-hash")
			.help("Template to use for outputted meta information. See docs for details"))
		.arg(Arg::new("padding")
			.short('p')
			.long("padding")
			.value_parser(clap::value_parser!(i32))
			.action(clap::ArgAction::Set)
			.default_value("2")
			.help("Pixel padding inbetween subimages"))
		.arg(Arg::new("input")
			.help("Image filenames to add to atlas")
			.required(true)
			.num_args(1..)
			.action(clap::ArgAction::Append)
			.index(1))
		.arg(Arg::new("input-name-root-dir")
			.long("input-name-root-dir")
			.action(clap::ArgAction::Set)
			.default_value("")
			.help("Root directory to use for all relative input paths in the meta data"))
		.arg(Arg::new("output-name-root-dir")
			.long("output-name-root-dir")
			.action(clap::ArgAction::Set)
			.default_value("")
			.help("Root directory to use for all relative output paths in the meta data"))
		.get_matches();

	let raw_filenames = matches.get_many::<String>("input").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>();
	let input_filenames: Vec<PathBuf> = parse_input_filenames::parse(raw_filenames)?;
	let output_width = *matches.get_one::<i32>("width").unwrap();
	let output_height = *matches.get_one::<i32>("height").unwrap();
	let padding = *matches.get_one::<i32>("padding").unwrap();
	let meta_template = matches.get_one::<String>("meta-template").unwrap();
	let output_name_root_dir = std::path::Path::new(matches.get_one::<String>("output-name-root-dir").unwrap());
	let input_name_root_dir = std::path::Path::new(matches.get_one::<String>("input-name-root-dir").unwrap());
	let output_filename = std::path::Path::new(matches.get_one::<String>("image-output").unwrap());
	let output_meta_filename = matches.get_one::<String>("meta-output").unwrap();
	let allow_rotation = !matches.get_flag("rotation-disable");
	let allow_grow = !matches.get_flag("fixed-size");

	let mut packer = packer::Packer::new( output_width, output_height, allow_grow, allow_rotation, padding );

	println!( "Calculating rects..." );
	let mut inputs: Vec<inputimage::InputImage> = vec!();
	for filename in input_filenames.iter() {
		let mut input = inputimage::InputImage::load( filename );
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
		output_meta.add_input( input_name_root_dir,  &input, pack_result.rect.x, pack_result.rect.y, pack_result.rotated );
	}
	println!( "Outputting final image {:?}", output_filename );
	output.save( output_filename )?;

	let output_json_filename: std::path::PathBuf;
	if output_meta_filename.len() > 0 {
		// meta output name was specified on command line, use that
		output_json_filename = std::path::Path::new(output_meta_filename).to_owned();
	} else {
		// use file extension of template used for default
		// TODO: detect & parse out hypen to work out file extension
		if meta_template.starts_with("json-") {
			output_json_filename = std::path::Path::new("out").with_extension("json");
		} else {
			output_json_filename = std::path::Path::new("out").with_extension(meta_template.to_owned());
		}
	}
	output_meta.save( &output_json_filename, meta_template, output_name_root_dir, output_filename, output.w, output.h )?;
	Ok(())
}


fn main() {
	let result = operate();
	match result {
		Err( e ) => {
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
		Ok(_json) => println!("Complete!")
	}
}

