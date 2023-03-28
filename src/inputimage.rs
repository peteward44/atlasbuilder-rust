extern crate image;

pub struct InputImage {
	pub name: String,
	pub data: Vec<u8>,
	pub w: i32,
	pub h: i32,
	pub vx: i32, // trim coords
	pub vy: i32,
	pub vw: i32, // width and height after trimming
	pub vh: i32
}


impl InputImage {
	pub fn load( filename: &str ) -> InputImage {
		let imga = image::open( filename ).unwrap();
		let img: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> = imga.into_rgba8();
		let dims = img.dimensions();
		let data = img.into_vec();
		let w = dims.0 as i32;
		let h = dims.1 as i32;
		InputImage { name: filename.to_string(), w: w, h: h, vw: w, vh: h, vx: 0, vy: 0, data: data }
	}
	
	pub fn trim( &mut self ) {
		let mut left = 0;
		let mut right = 0;
		for row in 0..self.h {
			// find leftmost pixel
			for x in 0..self.w {
				let alpha = self.data[((row*self.w + x) * 4 + 3) as usize];
				if alpha != 0 {
					if left == 0 || x < left {
						left = x;
					}
					break;
				}
			}

			// find rightmost pixel
			for x in (0..self.w).rev() {
				let alpha = self.data[((row*self.w + x) * 4 + 3) as usize];
				if alpha != 0 {
					if right == 0 || x > right {
						right = x;
					}
					break;
				}
			}
		}

		let mut top = 0;
		let mut bottom = 0;
		for col in 0..self.w {
			// find topmost pixel
			for y in 0..self.h {
				let alpha = self.data[((y*self.w + col) * 4 + 3) as usize];
				if alpha != 0 {
					if top == 0 || y < top {
						top = y;
					}
					break;
				}
			}

			// find bottommost pixel
			for y in (0..self.h).rev() {
				let alpha = self.data[((y*self.w + col) * 4 + 3) as usize];
				if alpha != 0 {
					if bottom == 0 || y > bottom {
						bottom = y;
					}
					break;
				}
			}
		}
		self.vx = left;
		self.vw = right - left;
		self.vy = top;
		self.vh = bottom - top;
//		println!( "{:?} {:?} {:?} {:?}", left, right, top, bottom );
	}
}
