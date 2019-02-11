extern crate image;

use super::inputimage;


pub struct OutputImage {
	pub data: Vec<u8>,
	pub w: i32,
	pub h: i32
}

impl OutputImage {
	pub fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		for row in 0..img.vh {
			let srcx = (img.vy+row)*img.w*4 + img.vx*4;
			let srcy = srcx + img.w*4;
			let dstx = ( dy + row )*self.w*4 + dx*4;
			let dsty = dstx + img.w*4;
			self.data[dstx as usize..dsty as usize].copy_from_slice( &img.data[srcx as usize..srcy as usize] );
		}
	}

	pub fn new( w: i32, h: i32 ) -> OutputImage {
		let size = w*h*4;
		OutputImage { w: w, h: h, data: vec![0; size as usize] }
	}
	
	pub fn save( &self, filename: &str ) {
		image::save_buffer( filename, &self.data, 600, 600, image::RGBA(8)).unwrap();
	}
}
