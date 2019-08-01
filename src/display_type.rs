
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct DisplayType<'a, T>(&'a T, &'static str) where T: Display;

impl<'a, T> DisplayType<'a, T> where T: Display {
	#[inline]
	pub const fn new(d: &'a T, str: &'static str) -> Self {
		DisplayType(d, str)
	}
	
	#[inline(always)]
	pub fn data(self) -> &'a T {
		self.0
	}
}


impl<'a, T> Display for DisplayType<'a, T> where T: Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)?;
		f.write_str(self.1)
	}
}