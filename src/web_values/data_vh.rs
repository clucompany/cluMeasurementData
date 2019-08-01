
use crate::screen::ScreenResolution;
use crate::values::DataPxf;
use crate::values::ToPxf;
use std::fmt;
use std::fmt::Display;
use crate::screen::ElementResolution;
use crate::values::DataPx;
use crate::values::AccurateType;

pub trait ToVH<T, R> where T: AccurateType, R: AccurateType + ToPxf<R> {
	fn to_vh(self, px: DataPx<T>) -> DataVH<R> where Self: Sized;
	fn new_vh(&self, px: DataPx<T>) -> DataVH<R> where Self: Sized;
}


impl<T, T2> ToVH<T2, u64> for ElementResolution<T> where T: AccurateType, T2: AccurateType {	
	fn to_vh(self, px: DataPx<T2>) -> DataVH<u64> where Self: Sized {
		let px_data = px.to_px();
		let px_resolution = self.to_px();
		
		let mut result: u64 = 100u64;
		
		result = result.div(px_data.v_tou64());
		result = result.mul(px_resolution.v_tou64());
		
		DataVH::raw_new(
			result.to_pxf()
		)
	}
	
	fn new_vh(&self, px: DataPx<T2>) -> DataVH<u64> where Self: Sized {
		let px_data = px.to_px();
		let px_resolution = self.as_px();
		
		let mut result: u64 = 100u64;
		
		result = result.div(px_data.v_tou64());
		result = result.mul(px_resolution.v_tou64());
		
		DataVH::raw_new(
			result.to_pxf()
		)
	}
}

impl<T, T2> ToVH<T2, f64> for ElementResolution<T> where T: AccurateType, T2: AccurateType {	
	fn to_vh(self, px: DataPx<T2>) -> DataVH<f64> where Self: Sized {
		let px_data = px.to_px();
		let px_resolution = self.to_px();
		
		let mut result: f64 = 100f64;
		
		result = result.div(px_data.v_tof64());
		result = result.mul(px_resolution.v_tof64());
		
		DataVH::raw_new(
			result.to_pxf()
		)
	}
	
	fn new_vh(&self, px: DataPx<T2>) -> DataVH<f64> where Self: Sized {
		let px_data = px.to_px();
		let px_resolution = self.as_px();
		
		let mut result: f64 = 100f64;
		
		result = result.div(px_data.v_tof64());
		result = result.mul(px_resolution.v_tof64());
		
		DataVH::raw_new(
			result.to_pxf()
		)
	}
}


impl<T, T2> ToVH<T2, u64> for ScreenResolution<T> where T: AccurateType, T2: AccurateType {	
	fn to_vh(self, px: DataPx<T2>) -> DataVH<u64> where Self: Sized {
		self.height().to_vh(px)
	}
	
	fn new_vh(&self, px: DataPx<T2>) -> DataVH<u64> where Self: Sized {
		self.as_height().new_vh(px)
	}
}

impl<T, T2> ToVH<T2, f64> for ScreenResolution<T> where T: AccurateType, T2: AccurateType {	
	fn to_vh(self, px: DataPx<T2>) -> DataVH<f64> where Self: Sized {
		self.height().to_vh(px)
	}
	
	fn new_vh(&self, px: DataPx<T2>) -> DataVH<f64> where Self: Sized {
		self.as_height().new_vh(px)
	}
}


#[derive(Debug, Clone)]
pub struct DataVH<T>(DataPxf<T>) where T: AccurateType;

impl<T> Display for DataVH<T> where T: AccurateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.display_fmt(f)?;
		f.write_str("vh")
	}
}

impl<T> DataVH<T> where T: AccurateType {
	#[inline]
	fn raw_new(px: DataPxf<T>) -> Self {
		DataVH(px)
	}
	
	#[inline(always)]
	pub fn data(self) -> DataPxf<T> {
		self.0
	}
	
	#[inline(always)]
	pub fn clone_data(&self) -> DataPxf<T> {
		self.0.clone()
	}
	
	#[inline(always)]
	pub fn clone(&self) -> DataVH<T> {
		Self::raw_new(self.0.clone())
	}
}

