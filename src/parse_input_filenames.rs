use std::path::{ Path, PathBuf };
use std::fs;


fn is_image_file( p: &PathBuf ) -> bool {
	match p.extension() {
		Some( ext ) => {
			let lc = ext.to_str().unwrap_or( "" ).to_lowercase();
			lc == "jpg" || lc == "jpeg" || lc == "png"
		}
		None => false
	}
}

fn examine_dir( parent: &PathBuf, mut result: &mut Vec<PathBuf> ) -> Result<(), failure::Error> {
	for entry_ in fs::read_dir( parent )? {
		let p = entry_?.path();
		if p.is_file() {
			if is_image_file( &p ) {
				result.push( p );
			}
		} else if p.is_dir() {
			examine_dir( &p, &mut result )?;
		}
	}
	Ok(())
}

pub fn parse( inputs: Vec<&str> ) -> Result<Vec<PathBuf>, failure::Error> {
	let mut result : Vec<PathBuf> = vec!();
	
	for input in inputs.iter() {
		let p = Path::new( input.clone() ).to_path_buf();
		if p.is_dir() { 
			examine_dir( &p, &mut result )?;
		} else {
			if is_image_file( &p ) {
				result.push( p );
			}
		}
	}
	
	Ok(result)
}
