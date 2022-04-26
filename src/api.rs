extern crate oxipng;

use anyhow::Result;
use flutter_rust_bridge::ZeroCopyBuffer;
use oxipng::{optimize_from_memory, Headers, Options};

pub fn compress_png_image(
	bytes: Vec<u8>,
	preset: Option<u8>,
	max_compression: Option<bool>,
) -> Result<ZeroCopyBuffer<Vec<u8>>> {
	if let Some(yes) = max_compression {
		if yes {
			let mut option = Options::max_compression();
			option.strip = Headers::Safe;
			let compressed_bytes = optimize_from_memory(&bytes, &option)?;
			return Ok(ZeroCopyBuffer(compressed_bytes));
		}
	}

	if let Some(value) = preset {
		let mut option = Options::from_preset(value);
		option.strip = Headers::Safe;
		let compressed_bytes = optimize_from_memory(&bytes, &option)?;
		return Ok(ZeroCopyBuffer(compressed_bytes));
	}

	let mut option = Options::from_preset(2);
	option.strip = Headers::Safe;
	let compressed_bytes = optimize_from_memory(&bytes, &option)?;
	Ok(ZeroCopyBuffer(compressed_bytes))
}
