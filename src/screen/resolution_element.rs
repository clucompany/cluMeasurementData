
use crate::values::ToPx;
use crate::values::DataPx;
use crate::values::AccurateType;

pub trait ToElementResolution<R> where R: AccurateType {
	fn to_element_resolution(self) -> ElementResolution<R> where Self: Sized;
	fn new_element_resolution(&self) -> ElementResolution<R> where Self: Sized;
}

impl ToElementResolution<Self> for u32 {
	#[inline(always)]
	fn to_element_resolution(self) -> ElementResolution<Self> {
		ElementResolution::raw_new(self.to_px())
	}
	
	#[inline(always)]
	fn new_element_resolution(&self) -> ElementResolution<Self> {
		ElementResolution::raw_new(self.new_px())
	}
}

impl ToElementResolution<Self> for u64 {
	#[inline(always)]
	fn to_element_resolution(self) -> ElementResolution<Self> {
		ElementResolution::raw_new(self.to_px())
	}
	
	#[inline(always)]
	fn new_element_resolution(&self) -> ElementResolution<Self> {
		ElementResolution::raw_new(self.new_px())
	}
}


#[derive(Debug, Clone)]
pub struct ElementResolution<T>(DataPx<T>) where T: AccurateType;


impl<T> ElementResolution<T> where T: AccurateType {
	#[inline(always)]
	pub fn data(self)	-> DataPx<T> {
		self.0
	}
	
	#[inline(always)]
	pub fn as_data(&self) -> &DataPx<T> {
		&self.0
	}
	
	#[inline(always)]
	pub fn as_px(&self) -> &T {
		self.0.as_px()
	}
	
	#[inline(always)]
	pub fn to_px(self) -> T {
		self.0.to_px()
	}
	
	#[inline(always)]
	pub fn clone_data(&self) -> DataPx<T> {
		self.0.clone()
	}
}

impl<T> ElementResolution<T> where T: AccurateType {
	#[inline]
	fn raw_new(p: DataPx<T>) -> Self {
		ElementResolution(p)
	}
}

