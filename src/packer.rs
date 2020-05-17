use std::cmp;
use std::i32;
use super::shapes;

pub struct PackResult {
	pub rect: shapes::Rect,
	pub rotated: bool
}

// True if a is contained within b
fn rect_contains( a: &shapes::Rect, b: &shapes::Rect ) -> bool {
	a.x >= b.x && a.y >= b.y 
		&& a.x+a.w <= b.x+b.w 
		&& a.y+a.h <= b.y+b.h
}

fn rect_intersects( a: &shapes::Rect, b: &shapes::Rect ) -> bool {
	!( a.x >= b.x + b.w
		|| a.x + a.w <= b.x
		|| a.y >= b.y + b.h
		|| a.y + a.h <= b.y )
}

fn punch_hole_in_rect( parent: &shapes::Rect, hole: &shapes::Rect, new_rects: &mut Vec<shapes::Rect> ) {
	if hole.x < parent.x + parent.w && hole.x + hole.w > parent.x {
		// new node at top side of the used node.
		if hole.y > parent.y && hole.y < parent.y + parent.h {
			new_rects.push( shapes::Rect{
				x: parent.x,
				y: parent.y,
				w: parent.w,
				h: hole.y - parent.y
			} );
		}
		
		// New node at the bottom side of the used node
		if hole.y + hole.h < parent.y + parent.h {
			new_rects.push( shapes::Rect{
				x: parent.x,
				y: hole.y + hole.h,
				w: parent.w,
				h: parent.y + parent.h - hole.y - hole.h
			} );
		}
	}

	if hole.y < parent.y + parent.h && hole.y + hole.h > parent.y {
		// new node at the left side of the used node.
		if hole.x > parent.x && hole.x < parent.x + parent.w {
			new_rects.push( shapes::Rect{
				x: parent.x,
				y: parent.y,
				w: hole.x - parent.x,
				h: parent.h
			} );
		}
		
		// new node at the right side of the used node
		if hole.x + hole.w < parent.x + parent.w {
			new_rects.push( shapes::Rect{
				x: hole.x + hole.w,
				y: parent.y,
				w: parent.x + parent.w - hole.x - hole.w,
				h: parent.h
			} );
		}
	}
}

pub struct Packer {
	w: i32,
	h: i32,
	w_limit: i32,
	h_limit: i32,
	allow_rotate: bool,
	used_rects: Vec<shapes::Rect>,
	free_rects: Vec<shapes::Rect>,
	padding: i32,
	results: Vec<PackResult>
}

impl Packer {
	pub fn new( w: i32, h: i32, allow_grow: bool, allow_rotate: bool, padding: i32 ) -> Packer {
		let w_use = if allow_grow { 128 } else { w };
		let h_use = if allow_grow { 128 } else { h };
		let mut free = vec!();
		free.push( shapes::Rect{ x: padding, y: padding, w: w_use - padding, h: h_use - padding } );
		Packer{
			w: w_use,
			h: h_use,
			w_limit: w,
			h_limit: h,
			used_rects: vec!(),
			free_rects: free,
			allow_rotate: allow_rotate,
			padding: padding,
			results: vec!()
		}
	}
	
//	pub fn get_free_rects( &self ) -> &Vec<shapes::Rect> {
//		&self.free_rects
//	}

	fn find_best_free_rect( &self, w: i32, h: i32, free_rects: &Vec<shapes::Rect> ) -> Option<PackResult> {
		// Find best free rectangle to insert target rect into
		let mut best_short_side_fit = std::i32::MAX;
		let mut best_long_side_fit = std::i32::MAX;
		let mut best_rect: shapes::Rect = shapes::Rect{ x:0, y:0, w:0, h: 0 };
		let mut best_rotated: bool = false;
		let mut parent_width = 0;
		let mut parent_height = 0;
		
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
					parent_width = rect.w;
					parent_height = rect.h;
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
			debug!("No space in atlas");
			None
		} else {
			debug!("Found rect.x={:?} rect.y={:?} rect.w={:?} rect.h={:?}", best_rect.x, best_rect.y, best_rect.w, best_rect.h);
			debug!("(Parent) rect.w={:?} rect.h={:?}", parent_width, parent_height );
			
			Some( PackResult{
				rect: best_rect, rotated: best_rotated
			} )
		}
	}

	fn attempt_pack( &self, w: i32, h: i32, free_rects: &mut Vec<shapes::Rect> ) -> Option<PackResult> {
		debug!( "attempt_pack w={:?} h={:?} self.padding={:?}", w, h, self.padding );
		let result_option = self.find_best_free_rect( w + self.padding, h + self.padding, free_rects );
		match result_option {
			Some( mut result ) => {
				let mut new_rects: Vec<shapes::Rect> = vec!();
				free_rects.retain( |free_rect| {
					if !rect_intersects( free_rect, &result.rect ) {
						return true;
					}
					punch_hole_in_rect( free_rect, &result.rect, &mut new_rects );
					return false;
				} );
				
				for rect in new_rects {
					free_rects.push( rect );
				}
				
				self.prune_free_rects( free_rects );
				
//				outputdebug::outputFreeRects( self.w, self.h, free_rects );
				
				debug!( "result.rect.x = {:?} result.rect.w = {:?} result.rect.y = {:?} result.rect.h = {:?}", result.rect.x, result.rect.w, result.rect.y, result.rect.h );
				result.rect.w -= self.padding;
				result.rect.h -= self.padding;
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
				if !removed.contains( &i ) && rect_contains( &free_rects[i], &free_rects[j] ) {
					removed.push( i );
					break;
				}
				if !removed.contains( &j ) && rect_contains( &free_rects[j], &free_rects[i] ) {
					removed.push( j );
				}
			}
		}
		removed.sort_unstable();
		for index in removed.iter().rev() {
			free_rects.remove( *index );
		}
	}
	
	pub fn add( &mut self, w: i32, h: i32 ) {
		self.used_rects.push( shapes::Rect{ x: 0, y: 0, w: w, h: h } );
	}
	
	pub fn pack( &mut self ) -> bool {
		let mut new_results: Vec<PackResult> = vec!();
		let mut free_rects: Vec<shapes::Rect> = vec!();
		free_rects.push( shapes::Rect{ x: self.padding, y: self.padding, w: self.w - self.padding, h: self.h - self.padding } );
		new_results.reserve( self.results.len() );
		for used_rect in self.used_rects.iter() {
			let result = self.attempt_pack( used_rect.w, used_rect.h, &mut free_rects );
			let cont = match result {
				None => {
					false
				},
				Some( resultx ) => {
				//	println!( "used rect x={:?} y={:?} w={:?} h={:?}", resultx.rect.x, resultx.rect.y, resultx.rect.w, resultx.rect.h );
					new_results.push( resultx );
					true
				},
			};
			if cont == false {
				return false;
			}
		}
		self.results = new_results;
		self.free_rects = free_rects;
		true
	}

	// Expensive operation
	pub fn grow( &mut self ) -> bool {
		loop {
			if self.w >= self.w_limit && self.h >= self.h_limit {
				return false;
			}
			if self.w >= self.w_limit {
				self.h *= 2;
//				println!( "self.h={:?}", self.h );
			} else if self.h >= self.h_limit {
				self.w *= 2;
//				println!( "self.w={:?}", self.w );
			} else {
				if self.w < self.h {
					self.w *= 2;
//					println!( "self.w={:?}", self.w );
				} else {
					self.h *= 2;
//					println!( "self.h={:?}", self.h );
				}
			}
			self.w = std::cmp::min( self.w, self.w_limit );
			self.h = std::cmp::min( self.h, self.h_limit );
			if self.pack() {
				break;
			}
		}
		
		true
	}
	
	pub fn get_results( &self ) -> &Vec<PackResult> {
		&self.results
	}
	
	pub fn get_w( &self ) -> i32 {
		self.w
	}
	
	pub fn get_h( &self ) -> i32 {
		self.h
	}
}

#[cfg(test)]
mod test_packer {

	fn assert_pack_result( result: &super::PackResult, x: i32, y: i32, w: i32, h: i32, rotated: bool, message: &str ) {
		println!( "x={:?} y={:?} w={:?} h={:?}", result.rect.x, result.rect.y, result.rect.w, result.rect.h );
		assert_eq!( result.rect.x, x, "{} - x", message );
		assert_eq!( result.rect.y, y, "{} - y", message );
		assert_eq!( result.rect.w, w, "{} - w", message );
		assert_eq!( result.rect.h, h, "{} - h", message );
		assert_eq!( result.rotated, rotated, "{}", message );
	}

	#[test]
	fn basic_packer_test() {
		let mut packer = super::Packer::new( 256, 256, false, false, 0 );

		packer.add( 10, 10 );
		let result1 = packer.pack();
		assert_eq!( result1, true );
		assert_pack_result( &packer.get_results()[0], 0, 0, 10, 10, false, "Test 1" );

		packer.add( 10, 10 );
		let result2 = packer.pack();
		assert_eq!( result2, true );
		assert_pack_result( &packer.get_results()[1], 0, 10, 10, 10, false, "Test 2" );

		packer.add( 50, 10 );
		let result3 = packer.pack();
		assert_eq!( result3, true );
		assert_pack_result( &packer.get_results()[2], 10, 0, 50, 10, false, "Test 3" );
			
		packer.add( 23, 75 );
		let result5 = packer.pack();
		assert_eq!( result5, true );
		assert_pack_result( &packer.get_results()[3], 0, 20, 23, 75, false, "Test 4" );

		packer.add( 50, 50 );
		let result4 = packer.pack();
		assert_eq!( result4, true );
		assert_pack_result( &packer.get_results()[4], 0, 95, 50, 50, false, "Test 5" );
	}
	
	#[test]
	fn automatic_grow_test() {
		let mut packer = super::Packer::new( 1024, 1024, true, false, 0 );
		assert_eq!( packer.w, 128 );
		assert_eq!( packer.h, 128 );
		packer.add( 200, 100 );
		let result1 = packer.pack();
		assert_eq!( result1, false );
		let grow_result = packer.grow();
		assert_eq!( grow_result, true );
		assert_eq!( packer.w, 256 );
		assert_eq!( packer.h, 256 );
		let result2 = packer.pack();
		assert_eq!( result2, true );
		assert_pack_result( &packer.get_results()[0], 0, 0, 200, 100, false, "Test 1" );
	}
	
	#[test]
	fn similar_shapes_test() {
		let rects = vec![
			super::shapes::Rect{ x: 0, y: 0, w: 304, h: 424 },
			super::shapes::Rect{ x: 0, y: 0, w: 300, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 304, h: 377 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 302, h: 377 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 181, h: 323 },
			super::shapes::Rect{ x: 0, y: 0, w: 300, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 178, h: 286 },
			super::shapes::Rect{ x: 0, y: 0, w: 300, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 216, h: 338 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 141, h: 329 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 378 },
			super::shapes::Rect{ x: 0, y: 0, w: 264, h: 318 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 379 },
			super::shapes::Rect{ x: 0, y: 0, w: 275, h: 369 },
			super::shapes::Rect{ x: 0, y: 0, w: 301, h: 382 },
			super::shapes::Rect{ x: 0, y: 0, w: 273, h: 367 },
			super::shapes::Rect{ x: 0, y: 0, w: 302, h: 396 }
		];
		let mut packer = super::Packer::new( 4096, 4096, false, true, 0 );
		for rect in rects {
			packer.add( rect.w, rect.h );
		}
		let pack_result = packer.pack();
		assert_eq!( pack_result, true );
		let results = packer.get_results();
		for result in results {
			debug!( "x={:?} y={:?} w={:?} h={:?}", result.rect.x, result.rect.y, result.rect.w, result.rect.h );
		}
		//assert_eq!( false, true );
	}
}


