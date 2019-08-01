
use std::ops::Deref;
use std::fmt::Display;
use crate::values::accurate::AccurateType;
use std::fmt;

pub trait ToPx<R> where R: AccurateType + ToPx<R> {
	fn to_px(self) -> DataPx<R>;
	fn new_px(&self) -> DataPx<R>;
}

impl ToPx<Self> for u32 {		
	#[inline(always)]
	fn to_px(self) -> DataPx<Self> {
		DataPx::raw_new(self)
	}
	
	#[inline(always)]
	fn new_px(&self) -> DataPx<Self> {
		DataPx::raw_new(*self)
	}
}

impl ToPx<Self> for u64 {		
	#[inline(always)]
	fn to_px(self) -> DataPx<Self> {
		DataPx::raw_new(self)
	}
	
	#[inline(always)]
	fn new_px(&self) -> DataPx<Self> {
		DataPx::raw_new(*self)
	}
}

impl ToPx<Self> for f32 {	
	#[inline(always)]
	fn to_px(self) -> DataPx<Self> {
		DataPx::raw_new(self)
	}
	
	#[inline(always)]
	fn new_px(&self) -> DataPx<Self> {
		DataPx::raw_new(*self)
	}
}

impl ToPx<Self> for f64 {	
	#[inline(always)]
	fn to_px(self) -> DataPx<Self> {
		DataPx::raw_new(self)
	}
	
	#[inline(always)]
	fn new_px(&self) -> DataPx<Self> {
		DataPx::raw_new(*self)
	}
}



#[derive(Debug, Clone)]
pub struct DataPx<T>(T) where T: AccurateType;

impl<T> Deref for DataPx<T> where T: AccurateType {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0	
	}
}	

impl<T> Display for DataPx<T> where T: AccurateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.display_fmt(f)?;
		f.write_str("px")
	}
}

impl<T> DataPx<T> where T: AccurateType {
	#[inline]
	const fn raw_new(data: T) -> Self {
		DataPx(data)
	}
	
	#[inline(always)]
	pub fn as_px(&self) -> &T {
		&self.0
	}
	
	#[inline(always)]
	pub fn to_px(self) -> T {
		self.0
	}
	
	#[inline(always)]
	pub fn clone_data(&self) -> T {
		self.0.clone_data()
	}
}




#[macro_export]
macro_rules! px {
	[$a:expr] => {
		crate::values::data::DataPx::to_px($a)
	};
}