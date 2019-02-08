use serde::{Deserialize, Serialize};
use serde_json::{Result, to_string};

#[derive(Serialize, Deserialize)]
pub struct Size {
	pub w: i32,
	pub h: i32
}

#[derive(Serialize, Deserialize)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}
