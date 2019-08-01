
use crate::values::data_px::DataPx;
use crate::values::data_px::ToPx;
use std::fmt::Display;
use std::ops::Deref;
use crate::values::accurate::AccurateType;
use std::fmt;

pub trait ToPxf<R> where R: AccurateType + ToPxf<R> {
	fn to_pxf(self) -> DataPxf<R>;
	fn new_pxf(&self) -> DataPxf<R>;
}


impl ToPxf<Self> for f32 {
	#[inline(always)]
	fn to_pxf(self) -> DataPxf<Self> {
		DataPxf::raw_new(self)
	}
	
	#[inline(always)]
	fn new_pxf(&self) -> DataPxf<Self> {
		DataPxf::raw_new(*self)
	}
}

impl ToPxf<Self> for f64 {
	#[inline(always)]
	fn to_pxf(self) -> DataPxf<Self> {
		DataPxf::raw_new(self)
	}
	
	#[inline(always)]
	fn new_pxf(&self) -> DataPxf<Self> {
		DataPxf::raw_new(*self)
	}
}

impl ToPxf<Self> for u32 {
	#[inline(always)]
	fn to_pxf(self) -> DataPxf<Self> {
		DataPxf::raw_new(self)
	}
	
	#[inline(always)]
	fn new_pxf(&self) -> DataPxf<Self> {
		DataPxf::raw_new(*self)
	}
}

impl ToPxf<Self> for u64 {
	#[inline(always)]
	fn to_pxf(self) -> DataPxf<Self> {
		DataPxf::raw_new(self)
	}
	
	#[inline(always)]
	fn new_pxf(&self) -> DataPxf<Self> {
		DataPxf::raw_new(*self)
	}
}


#[derive(Debug, Clone)]
pub struct DataPxf<T>(T) where T: AccurateType;

impl<T> ToPxf<T> for DataPxf<T> where T: AccurateType, T: ToPxf<T> {
	#[inline(always)]
	fn to_pxf(self) -> DataPxf<T> {
		self
	}
	
	#[inline(always)]
	fn new_pxf(&self) -> DataPxf<T> {
		self.clone()
	}
}

impl<T> ToPx<T::U> for DataPxf<T> where T: AccurateType {
	fn to_px(self) -> DataPx<T::U> {
		self.as_data().v_tou().to_px()
	}
	fn new_px(&self) -> DataPx<T::U> {
		self.as_data().v_tou().to_px()
	}
}

impl<T> Deref for DataPxf<T> where T: AccurateType {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0	
	}
}	

impl<T> Display for DataPxf<T> where T: AccurateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.display_fmt(f)?;
		f.write_str("pxf")
	}
}

impl<T> DataPxf<T> where T: AccurateType {
	#[inline]
	const fn raw_new(data: T) -> Self {
		DataPxf(data)	
	}
	
	#[inline(always)]
	pub fn as_data(&self) -> &T {
		&self.0
	}
	
	#[inline(always)]
	pub fn clone_data(&self) -> T {
		self.0.clone_data()
	}
	
	#[inline(always)]
	pub fn clone(&self) -> DataPxf<T> {
		Self::raw_new(self.0.clone_data())
	}
	
	pub fn to_px(self) -> DataPx<T::U> {
		self.as_data().v_tou().to_px()
	}
}
