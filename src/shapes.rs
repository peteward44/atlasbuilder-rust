use serde::{Deserialize, Serialize};
use serde_json::{Result, to_string};

#[derive(Serialize, Deserialize, Copy)]
pub struct Size {
	pub w: i32,
	pub h: i32
}

impl Clone for Size {
	fn clone(&self) -> Size {
		*self
	}
}

#[derive(Serialize, Deserialize, Copy)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}

impl Clone for Rect {
	fn clone(&self) -> Rect {
		*self
	}
}