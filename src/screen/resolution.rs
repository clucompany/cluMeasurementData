
use crate::screen::resolution_element::ElementResolution;
use std::marker::PhantomData;
use crate::values::AccurateType;
use crate::screen::resolution_element::ToElementResolution;

pub trait ToScreenResolution<T> where T: AccurateType {
	fn to_screen_resolution(self) -> ScreenResolution<T> where Self: Sized;
	fn new_screen_resolution(&self) -> ScreenResolution<T> where Self: Sized;
}

impl<T> ToScreenResolution<T> for (T, T) where T: AccurateType + ToElementResolution<T> {
	fn to_screen_resolution(self) -> ScreenResolution<T> where Self: Sized {
		ScreenResolution::raw_new(self.0.to_element_resolution(), self.1.to_element_resolution())
	}
	
	fn new_screen_resolution(&self) -> ScreenResolution<T> where Self: Sized {
		ScreenResolution::raw_new(self.0.new_element_resolution(), self.1.new_element_resolution())
	}
}


#[derive(Debug, Clone)]
pub struct ScreenResolution<T> where T: AccurateType {
	width: ElementResolution<T>,
	height: ElementResolution<T>,
	
	_p: PhantomData<T>,
}

impl Default for ScreenResolution<u32> {
	#[inline(always)]
	fn default() -> Self {
		Self::resolution(1280u32, 800u32)
	}
}

impl Default for ScreenResolution<u64> {
	#[inline(always)]
	fn default() -> Self {
		Self::resolution(1280u64, 800u64)
	}
}


impl<T> ScreenResolution<T> where T: AccurateType {
	const fn raw_new(w: ElementResolution<T>, h: ElementResolution<T>) -> Self {
		Self {
			width: w,
			height: h,
			
			_p: PhantomData,
		}
	}
	
	#[inline]
	pub fn resolution<C, B>(w: C, h: B) -> Self where C: ToElementResolution<T>, B: ToElementResolution<T> {
		Self::_resolution(w.to_element_resolution(), h.to_element_resolution())
	}
	
	#[inline(always)]
	pub const fn _resolution(w: ElementResolution<T>, h: ElementResolution<T>) -> Self {
		Self::raw_new(w, h)
	}
	
	#[inline(always)]
	pub const fn as_width(&self) -> &ElementResolution<T> {
		&self.width
	}
	
	#[inline(always)]
	pub const fn as_height(&self) -> &ElementResolution<T> {
		&self.height
	}
	
	#[inline(always)]
	pub fn width(self) -> ElementResolution<T> {
		self.width	
	}
	
	#[inline(always)]
	pub fn height(self) -> ElementResolution<T> {
		self.height
	}
	
	
	
	#[inline(always)]
	pub fn data(self) -> (ElementResolution<T>, ElementResolution<T>) {
		(self.height, self.width)
	}
}

#[macro_export]
macro_rules! screen_resolution {
	[$a:tt x b:tt] => {
		crate::screen::ScreenResolution::px($a, $b)
	};
}

