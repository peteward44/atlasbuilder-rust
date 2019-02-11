use std::cmp;
use std::i32;

use super::inputimage;
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
	used_rects: Vec<shapes::Rect>,
	free_rects: Vec<shapes::Rect>
}

impl Packer {
	pub fn new( w: i32, h: i32 ) -> Packer {
		let mut free = vec!();
		free.push( shapes::Rect{ x: 0, y:0, w: w, h: h } );
		Packer{
			w: w,
			h: h,
			used_rects: vec!(),
			free_rects: free
		}
	}

	fn find_best_free_rect( &self, w: i32, h: i32, allow_rotate: bool ) -> PackResult {
		// Find best free rectangle to insert target rect into
		let mut best_short_side_fit = std::i32::MAX;
		let mut best_long_side_fit = std::i32::MAX;
		let mut best_rect: shapes::Rect = shapes::Rect{ x:0, y:0, w:0, h: 0 };
		let mut best_rotated: bool = false;
		
		for rect in self.free_rects.iter() {
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
			if allow_rotate && rect.w >= h && rect.h >= w {
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
		PackResult{
			rect: best_rect, rotated: best_rotated
		}
	}

	pub fn pack( &mut self, w: i32, h: i32, allow_rotate: bool ) -> PackResult {
		let result = self.find_best_free_rect( w, h, allow_rotate );
		let mut new_rects: Vec<shapes::Rect> = vec!();
		self.free_rects.retain( |free_rect| {
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
			self.free_rects.push( rect );
		}
		
		self.prune_free_rects();
		
		result
	}
	
	fn prune_free_rects( &mut self ) {
		let mut removed: Vec<usize> = vec!();
		for i in 0..self.free_rects.len() {
			for j in (i+1)..self.free_rects.len() {
				if rect_contains( &self.free_rects[i], &self.free_rects[j] ) {
					removed.push( i );
					break;
				}
				if rect_contains( &self.free_rects[j], &self.free_rects[i] ) {
					removed.push( j );
				}
			}
		}
		let mut index: usize = 0;
		self.free_rects.retain( |free_rect| {
			index = index + 1;
			!removed.contains( &index )
		} );
	}
}
