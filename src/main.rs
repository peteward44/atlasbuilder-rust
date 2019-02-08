#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod inputimage;
mod outputmeta;
mod outputimage;
mod shapes;


fn main() {
	let mut output_meta = outputmeta::OutputMeta::new();
	let mut output = outputimage::OutputImage::new( 600, 600 );
	let mut input = inputimage::InputImage::load( "test_images/input1_trim.png" );
	input.trim();
	
	output.add_input( &input, 0, 0 );
	output_meta.add_input( &input, 0, 0 );
	
	output.save( "test_images/out.png" );
	output_meta.save( "test_images/out.json", "hash" );
}

