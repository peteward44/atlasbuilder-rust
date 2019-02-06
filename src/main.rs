extern crate image;

use image::{GenericImageView};

fn main() {
//	let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = image::ImageBuffer::new(512, 512);
	
	const outpLength: usize = 600*600*4;
	let mut outp: Vec<u8> = vec![0; outpLength];
	//let mut outp: [u8; outpLength] = [0; outpLength]; // Generate the image data
	//let mut outp = [u8; outpLength];
	let mut imga = image::open( "test/input1.png" ).unwrap();
	let img: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> = imga.to_rgba();
	
	let inp = img.into_vec();
	//let mut outp = output.raw_pixels();
	
	for row in 0..200 {
		let srcx = row*200*4;
		let srcy = (row+1)*200*4;
		let dstx = row*600*4;
		let dsty = (row)*600*4 + 200*4;
		outp[dstx as usize..dsty as usize].copy_from_slice( &inp[srcx as usize..srcy as usize] );
	}

	//let img = ImageBuffer::new(512, 512);
	println!("Hello, world!");
	//println!("dimensions {:?}", img.dimensions());

	image::save_buffer( "test/out.png", &outp, 600, 600, image::RGBA(8)).unwrap();
//	output.save( "test/out.png" );
}

