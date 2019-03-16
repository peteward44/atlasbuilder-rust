use std::cmp;
use std::i32;
use super::shapes;

pub struct PackResult {
	pub rect: shapes::Rect,
	pub rotated: bool
}

fn rect_contains( a: &shapes::Rect, b: &shapes::Rect ) -> bool {
	return a.x >= b.x && a.y >= b.y 
		&& a.x+a.w <= b.x+b.w 
		&& a.y+a.h <= b.y+b.h
}

fn rect_intersects( a: &shapes::Rect, b: &shapes::Rect ) -> bool {
	return !( a.x >= b.x + b.w
		|| a.x + a.w <= b.x
		|| a.y >= b.y + b.h
		|| a.y + a.h <= b.y )
}

pub struct Packer {
	w: i32,
	h: i32,
	w_limit: i32,
	h_limit: i32,
	allow_rotate: bool,
	free_rects: Vec<shapes::Rect>,
	results: Vec<PackResult>
}

impl Packer {
	pub fn new( w: i32, h: i32, allow_grow: bool, allow_rotate: bool ) -> Packer {
		let w_use = if allow_grow { 128 } else { w };
		let h_use = if allow_grow { 128 } else { h };
		let mut free = vec!();
		free.push( shapes::Rect{ x: 0, y:0, w: w_use, h: h_use } );
		Packer{
			w: w_use,
			h: h_use,
			w_limit: w,
			h_limit: h,
			free_rects: free,
			allow_rotate: allow_rotate,
			results: vec!()
		}
	}

	fn find_best_free_rect( &self, w: i32, h: i32, free_rects: &Vec<shapes::Rect> ) -> Option<PackResult> {
		// Find best free rectangle to insert target rect into
		let mut best_short_side_fit = std::i32::MAX;
		let mut best_long_side_fit = std::i32::MAX;
		let mut best_rect: shapes::Rect = shapes::Rect{ x:0, y:0, w:0, h: 0 };
		let mut best_rotated: bool = false;
		
		for rect in free_rects.iter() {
			// Try to place the rectangle in upright (non-flipped) orientation
			if rect.w >= w && rect.h >= h {
				let leftover_horiz = ( rect.w - w ).abs();
				let leftover_vert = ( rect.h - h ).abs();
				let short_side_fit = cmp::min( leftover_horiz, leftover_vert );
				let long_side_fit = cmp::max( leftover_horiz, leftover_vert );
				
				if short_side_fit < best_short_side_fit || ( short_side_fit == best_short_side_fit && long_side_fit < best_long_side_fit ) {
					best_rect = shapes::Rect{
						x: rect.x, y: rect.y, w: w, h: h
					};
					best_short_side_fit = short_side_fit;
					best_long_side_fit = long_side_fit;
					best_rotated = false;
				}
			}

			// then try fitting it in rotated
			if self.allow_rotate && rect.w >= h && rect.h >= w {
				let leftover_horiz = ( rect.w - h ).abs();
				let leftover_vert = ( rect.h - w ).abs();
				let short_side_fit = cmp::min( leftover_horiz, leftover_vert );
				let long_side_fit = cmp::max( leftover_horiz, leftover_vert );
				
				if short_side_fit < best_short_side_fit || ( short_side_fit == best_short_side_fit && long_side_fit < best_long_side_fit ) {
					best_rect = shapes::Rect{
						x: rect.x, y: rect.y, w: h, h: w
					};
					best_short_side_fit = short_side_fit;
					best_long_side_fit = long_side_fit;
					best_rotated = true;
				}
			}
		}
		if best_rect.w == 0 {
			println!("No space in atlas");
			None
		} else {
			println!("Found rect.x={:?} rect.y={:?} rect.w={:?} rect.h={:?}", best_rect.x, best_rect.y, best_rect.w, best_rect.h);
			Some( PackResult{
				rect: best_rect, rotated: best_rotated
			} )
		}
	}

	fn attempt_pack( &self, w: i32, h: i32, free_rects: &mut Vec<shapes::Rect> ) -> Option<PackResult> {
		let result_option = self.find_best_free_rect( w, h, free_rects );
		match result_option {
			Some( result ) => {
				let mut new_rects: Vec<shapes::Rect> = vec!();
				free_rects.retain( |free_rect| {
					if !rect_intersects( free_rect, &result.rect ) {
						return true;
					}
					if result.rect.x < free_rect.x + free_rect.w && result.rect.x + result.rect.w > free_rect.x {
						// new node at top side of the used node.
						if result.rect.y > free_rect.y && result.rect.y < free_rect.y + free_rect.h {
							new_rects.push( shapes::Rect{
								x: free_rect.x,
								y: free_rect.y,
								w: free_rect.w,
								h: result.rect.y - free_rect.y
							} );
						}
						
						// New node at the bottom side of the used node
						if result.rect.y + result.rect.h < free_rect.y + free_rect.h {
							new_rects.push( shapes::Rect{
								x: free_rect.x,
								y: result.rect.y + result.rect.h,
								w: free_rect.w,
								h: free_rect.y + free_rect.h - result.rect.y - result.rect.h
							} );
						}
					}
					
					if result.rect.y < free_rect.y + free_rect.h && result.rect.y + result.rect.h > free_rect.y {
						// new node at the left side of the used node.
						if result.rect.x > free_rect.x && result.rect.x < free_rect.x + free_rect.w {
							new_rects.push( shapes::Rect{
								x: free_rect.x,
								y: free_rect.y,
								w: result.rect.x - free_rect.x,
								h: free_rect.h
							} );
						}
						
						// new node at the right side of the used node
						if result.rect.x + result.rect.w < free_rect.x + free_rect.w {
							new_rects.push( shapes::Rect{
								x: result.rect.x + result.rect.w,
								y: free_rect.y,
								w: free_rect.x + free_rect.w - result.rect.x - result.rect.w,
								h: free_rect.h
							} );
						}
					}
					return false;
				} );
				
				for rect in new_rects {
					free_rects.push( rect );
				}
				
				self.prune_free_rects( free_rects );
				
				println!( "result.rect.x = {:?} result.rect.w = {:?} result.rect.y = {:?} result.rect.h = {:?}", result.rect.x, result.rect.w, result.rect.y, result.rect.h );
				Some( result )
			}
			None => {
				None
			}
		}
	}
	
	fn prune_free_rects( &self, free_rects: &mut Vec<shapes::Rect> ) {
		let mut removed: Vec<usize> = vec!();
		for i in 0..free_rects.len() {
			for j in (i+1)..free_rects.len() {
				if rect_contains( &free_rects[i], &free_rects[j] ) {
					removed.push( i );
					break;
				}
				if rect_contains( &free_rects[j], &free_rects[i] ) {
					removed.push( j );
				}
			}
		}
		let mut index: usize = 0;
		free_rects.retain( |_free_rect| {
			index = index + 1;
			!removed.contains( &index )
		} );
	}
	
	pub fn pack( &mut self, w: i32, h: i32 ) -> bool {
		let mut rects: Vec<shapes::Rect> = self.free_rects.clone();
		let result = self.attempt_pack( w, h, &mut rects );
		match result {
			Some(result) => {
				self.free_rects = rects;
				self.results.push( result );
				true
			}
			None => {
				false
			}
		}
	}
	
	fn repack_results( &mut self ) -> bool {
		let mut new_results: Vec<PackResult> = vec!();
		let mut free_rects: Vec<shapes::Rect> = vec!();
		free_rects.push( shapes::Rect{ x: 0, y:0, w: self.w, h: self.h } );
		new_results.reserve( self.results.len() );
		for result in self.results.iter() {
			let result = self.attempt_pack( if result.rotated { result.rect.h } else { result.rect.w }, if result.rotated { result.rect.w } else { result.rect.h }, &mut free_rects );
			match result {
				Some(resultx) => {
					new_results.push( resultx );
				}
				None => {
					return false;
				}
			}
		}
		self.results = new_results;
		self.free_rects = free_rects;
		return true;
	}

	// Expensive operation
	pub fn grow( &mut self ) -> bool {
		loop {
			if self.w >= self.w_limit && self.h >= self.h_limit {
				return false;
			}
			if self.w >= self.w_limit {
				self.h *= 2;
				println!( "self.h={:?}", self.h );
			} else if self.h >= self.h_limit {
				self.w *= 2;
				println!( "self.w={:?}", self.w );
			} else {
				if self.w < self.h {
					self.w *= 2;
					println!( "self.w={:?}", self.w );
				} else {
					self.h *= 2;
					println!( "self.h={:?}", self.h );
				}
			}
			self.w = std::cmp::min( self.w, self.w_limit );
			self.h = std::cmp::min( self.h, self.h_limit );
			if self.repack_results() {
				break;
			}
		}
		
		true
	}
	
	pub fn get_results( &self ) -> &Vec<PackResult> {
		&self.results
	}
}

#[cfg(test)]
mod test_packer {

	fn assert_pack_result( result: &super::PackResult, x: i32, y: i32, w: i32, h: i32, rotated: bool ) {
		println!( "{:?} {:?}", result.rect.x, result.rect.y );
		assert_eq!( result.rect.x, x );
		assert_eq!( result.rect.y, y );
		assert_eq!( result.rect.w, w );
		assert_eq!( result.rect.h, h );
		assert_eq!( result.rotated, rotated );
	}

	#[test]
	fn basic_packer_test() {
		let mut packer = super::Packer::new( 100, 100, false, false );

		let result1 = packer.pack( 10, 10 );
		assert_eq!( result1, true );
		assert_pack_result( &packer.get_results()[0], 0, 0, 10, 10, false );

		let result2 = packer.pack( 10, 10 );
		assert_eq!( result2, true );
		assert_pack_result( &packer.get_results()[1], 0, 10, 10, 10, false );

		let result3 = packer.pack( 50, 10 );
		assert_eq!( result3, true );
		assert_pack_result( &packer.get_results()[2], 10, 10, 50, 10, false );
		
		let result4 = packer.pack( 50, 50 );
		assert_eq!( result4, true );
		assert_pack_result( &packer.get_results()[3], 10, 20, 50, 50, false );
		
		let result5 = packer.pack( 23, 75 );
		assert_eq!( result5, true );
		assert_pack_result( &packer.get_results()[4], 60, 20, 23, 75, false );
	}
	
	#[test]
	fn automatic_grow_test() {
		let mut packer = super::Packer::new( 1024, 1024, true, false );
		assert_eq!( packer.w, 128 );
		assert_eq!( packer.h, 128 );
		let result1 = packer.pack( 200, 100 );
		assert_eq!( result1, false );
		let grow_result = packer.grow();
		assert_eq!( grow_result, true );
		assert_eq!( packer.w, 128 );
		assert_eq!( packer.h, 256 );
		let result2 = packer.pack( 100, 200 );
		assert_eq!( result2, true );
		assert_pack_result( &packer.get_results()[0], 0, 0, 100, 200, false );
	}
}


