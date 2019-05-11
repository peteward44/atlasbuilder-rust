extern crate image;
extern crate rand;

use rand::distributions::{Distribution, Uniform};
use super::shapes;

fn write_pixel( data: &mut Vec<u8>, x: i32, y: i32, pitch: i32, r: u8, g: u8, b: u8 ) {
	let pos = (x + (y * pitch))*4;
	data[(pos+0) as usize] = r;
	data[(pos+1) as usize] = g;
	data[(pos+2) as usize] = b;
	data[(pos+3) as usize] = 255;
}

fn draw_rect( data: &mut Vec<u8>, pitch: i32, rect: &shapes::Rect ) {
    let mut rng = rand::thread_rng();
	let range = Uniform::from(0..255);

	let r = range.sample( &mut rng ) as u8;
	let g = range.sample( &mut rng ) as u8;
	let b = range.sample( &mut rng ) as u8;
	for x in 0..rect.w {
		// top line
		let pos = x + rect.x + (rect.y * pitch);
		if pos >= data.len() as i32 {
			println!( "out of bounds {:?} ({:?})", pos, data.len() );
			return;
		}
		write_pixel( data, x + rect.x, rect.y, pitch, r, g, b );
		write_pixel( data, x + rect.x, rect.y + 1, pitch, r, g, b );

		// bottom line
		write_pixel( data, x + rect.x, rect.y + rect.h - 1, pitch, r, g, b );
		write_pixel( data, x + rect.x, rect.y + rect.h - 2, pitch, r, g, b );
	}

	for y in 0..rect.h {
		// left line
		write_pixel( data, rect.x, rect.y + y, pitch, r, g, b );
		write_pixel( data, rect.x + 1, rect.y + y, pitch, r, g, b );
		
		// right line
		write_pixel( data, rect.x + rect.w - 1, rect.y + y, pitch, r, g, b );
		write_pixel( data, rect.x + rect.w - 2, rect.y + y, pitch, r, g, b );
	}
	
}

pub fn output_free_rects( w: i32, h: i32, free_rects: &mut Vec<shapes::Rect>, index: i32 ) -> std::result::Result<(), failure::Error> {
	let filename = format!( "debug{:?}.png", index );
	let size = w*h*4;
	let mut data: Vec<u8> = vec![0; size as usize];
	for rect in free_rects {
		println!( "w={:?} h={:?} rect.x={:?} rect.y={:?} rect.w={:?} rect.h={:?}", w, h, rect.x, rect.y, rect.w, rect.h );
		draw_rect( &mut data, w, &rect,  );
	}
	image::save_buffer( filename, &data, w as u32, h as u32, image::RGBA(8))?;
	Ok(())
}
