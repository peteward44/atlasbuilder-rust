extern crate image;

use image::{DynamicImage, GenericImageView};

struct Rect {
	x: i32,
	y: i32,
	w: i32,
	h: i32
}

struct InputImage {
	data: Vec<u8>,
	width: i32,
	height: i32
}


impl InputImage {
    fn new( w: i32, h: i32 ) -> InputImage {
        InputImage { width: w, height: h, data: vec!() }
    }
}

struct OutputImage {
	data: Vec<u8>,
	width: i32,
	height: i32,
	rects: Vec<Rect>
}

impl OutputImage {
    fn add_input( &mut self, img: InputImage, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32 ) {
		println!("add_input");
		for row in 0..h {
			let srcx = row*200*4;
			let srcy = (row+1)*200*4;
			let dstx = row*600*4;
			let dsty = (row)*600*4 + 200*4;
			//outp[dstx as usize..dsty as usize].copy_from_slice( &inp[srcx as usize..srcy as usize] );
		}
	}

    fn new( w: i32, h: i32 ) -> OutputImage {
        OutputImage { width: w, height: h, rects: vec!(), data: vec!() }
    }
}


fn main() {
	let mut output = OutputImage::new( 600, 600 );
	let input = InputImage::new( 200, 200 );
	output.add_input( input, 0, 0, 0, 0, input.width, input.height );

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

