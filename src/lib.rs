#![allow(non_snake_case)]
#![feature(const_fn)]

/* 29-30 06 2019 */

pub mod display_type;

#[macro_use]
pub mod screen {
	mod resolution;
	pub use self::resolution::*;
	
	mod resolution_element;
	pub use self::resolution_element::*;
}

#[macro_use]
pub mod values {
	mod accurate;
	pub use self::accurate::*;
	
	mod data_px;
	pub use self::data_px::*;
	
	mod data_pxf;
	pub use self::data_pxf::*;
}

#[macro_use]
pub mod web_values {
	mod data_vh;
	pub use self::data_vh::*;
}
