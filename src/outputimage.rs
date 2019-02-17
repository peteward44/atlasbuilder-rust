extern crate image;

use super::inputimage;


pub struct OutputImage {
	pub data: Vec<u8>,
	pub w: i32,
	pub h: i32
}

impl OutputImage {
	pub fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32, rotated: bool ) {
		if rotated {
			for row in 0..img.vh {
				let src_row = (img.vy+row)*(img.w*4) + img.vx*4;
				for col in 0..img.vw {
					let src = src_row + col*4;
					let dst_row = (dy+col)*self.w*4; // transpose col / row
					let dst = dst_row + (dx+row)*4;
					self.data[dst as usize..(dst+4) as usize].copy_from_slice( &img.data[src as usize..(src+4) as usize] );
				}
			}
		} else {
			for row in 0..img.vh {
				let srcx = (img.vy+row)*img.w*4 + img.vx*4;
				let srcy = srcx + img.w*4;
				let dstx = ( dy + row )*self.w*4 + dx*4;
				let dsty = dstx + img.w*4;
				self.data[dstx as usize..dsty as usize].copy_from_slice( &img.data[srcx as usize..srcy as usize] );
			}
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



#[cfg(test)]
mod test_outputimage {
	#[test]
	fn add_image() {
		let input_vec = vec![0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
							1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1,
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0 ];
		let mut output_image = super::OutputImage::new( 5, 5 );
		let input_image = super::inputimage::InputImage{ name: "test.png".to_string(), w: 5, h: 5, vw: 5, vh: 5, vx: 0, vy: 0, data: input_vec.clone() };
		output_image.add_input( &input_image, 0, 0, false );
		for x in 0..(5*5*4) {
			let pixel = x/4;
			assert_eq!( output_image.data[x], input_vec[x], "Test {}x{}", pixel/5, pixel%5 );
		}
	}

	#[test]
	fn add_rotated_image() {
		let input_vec = vec![0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
							1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1, 1,1,1,1,
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0 ];
		
		let rotated_vec = vec![0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0,
							0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0, 
							0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0, 0,0,0,0 ];
		let mut output_image = super::OutputImage::new( 5, 5 );
		let input_image = super::inputimage::InputImage{ name: "test.png".to_string(), w: 5, h: 5, vw: 5, vh: 5, vx: 0, vy: 0, data: input_vec };
		output_image.add_input( &input_image, 0, 0, true );
		for x in 0..rotated_vec.len() {
			let pixel = x/4;
			assert_eq!( output_image.data[x], rotated_vec[x], "Test {}x{}", pixel/5, pixel%5 );
		}
	}
}


