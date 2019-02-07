extern crate image;

mod inputimage;
mod outputmeta;


struct OutputImage {
	data: Vec<u8>,
	w: i32,
	h: i32
}

impl OutputImage {
	fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32 ) {
		for row in 0..img.vh {
			let srcx = (img.vy+row)*img.w*4 + img.vx*4;
			let srcy = srcx + img.w*4;
			let dstx = ( dy + row )*self.w*4 + dx*4;
			let dsty = dstx + img.w*4;
			self.data[dstx as usize..dsty as usize].copy_from_slice( &img.data[srcx as usize..srcy as usize] );
		}
	}

	fn new( w: i32, h: i32 ) -> OutputImage {
		let size = w*h*4;
		OutputImage { w: w, h: h, data: vec![0; size as usize] }
	}
	
	fn save( &self, filename: &str ) {
		image::save_buffer( filename, &self.data, 600, 600, image::RGBA(8)).unwrap();
	}
}

fn main() {
	let mut output_meta = outputmeta::OutputMeta::new();
	let mut output = OutputImage::new( 600, 600 );
	let mut input = inputimage::InputImage::load( "test/input1_trim.png" );
	input.trim();
	
	output.add_input( &input, 0, 0 );
	output_meta.add_input( &input, 0, 0 );
	
	output.save( "test/out.png" );
	output_meta.save( "test/out.json" );
}

