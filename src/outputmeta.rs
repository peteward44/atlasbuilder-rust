use super::inputimage;

struct SubImage {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
	pub vw: i32,
	pub vh: i32
}

pub struct OutputMeta {
	subs: Vec<SubImage>
}

impl OutputMeta {
	pub fn new() -> OutputMeta {
		OutputMeta { subs: vec!() }
	}

	pub fn add_input( &mut self, img: &inputimage::InputImage, dx: i32, dy: i32 ) {
		let rect = SubImage{ x: dx, y: dy, w: img.w, h: img.h, vw: img.vw, vh: img.vh };
		self.subs.push( rect );
	}
	
	pub fn save( &self, filename: &str ) {
		
	}
}
