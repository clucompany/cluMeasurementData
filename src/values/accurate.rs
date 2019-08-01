
use crate::values::data_pxf::ToPxf;
use crate::values::data_px::ToPx;
use std::fmt::Display;
use std::fmt;

pub trait AccurateType: Clone + Copy {
	type U: AccurateType<U=<Self as AccurateType>::U> + ToPx<<Self as AccurateType>::U>;
	type F: AccurateType<F=<Self as AccurateType>::F> + ToPxf<<Self as AccurateType>::F>;
	
	fn maybeNegative(&self) -> bool;
	fn numbersAfterComma(&self) -> bool;
	
	fn copy(&self) -> Self;
	fn clone_data(&self) -> Self;
	
	fn v_0() -> Self;
	fn v_100() -> Self;
	fn v_100f() -> Self::F;
	
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error>;
	
	fn v_tof(self) -> Self::F;
	fn v_tof64(self) -> f64;
	
	fn v_tou(self) -> Self::U;
	fn v_tou64(self) -> u64;
	
	fn mul(self, s: Self) -> Self;
	fn div(self, s: Self) -> Self;
}


impl AccurateType for f32 {
	type U = u32;
	type F = Self;
	
	#[inline(always)]
	fn copy(&self) -> Self {
		*self
	}
	
	#[inline(always)]
	fn clone_data(&self) -> Self {
		*self
	}
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		false
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		true
	}
	
	#[inline]
	fn v_0() -> Self {
		0f32
	}
	
	#[inline]
	fn v_100() -> Self {
		100f32
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f32
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline(always)]
	fn v_tof64(self) -> f64 {
		self as f64	
	}
	
	#[inline(always)]
	fn v_tou64(self) -> u64 {
		self as u64	
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}

impl AccurateType for f64 {
	type U = u64;
	type F = Self;
	
	#[inline(always)]
	fn copy(&self) -> Self {
		*self
	}
	
	#[inline(always)]
	fn clone_data(&self) -> Self {
		*self
	}
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		false	
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		true	
	}
	
	#[inline]
	fn v_0() -> Self {
		0f64
	}
	
	#[inline]
	fn v_100() -> Self {
		100f64
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f64
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self
	}
	
	#[inline(always)]
	fn v_tof64(self) -> f64 {
		self as f64	
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline(always)]
	fn v_tou64(self) -> u64 {
		self as u64
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}

impl AccurateType for u32 {
	type U = Self;
	type F = f32;
	
	#[inline(always)]
	fn copy(&self) -> Self {
		*self
	}
	
	#[inline(always)]
	fn clone_data(&self) -> Self {
		*self
	}
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		false	
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		false	
	}
	
	#[inline]
	fn v_0() -> Self {
		0u32
	}
	
	#[inline]
	fn v_100() -> Self {
		100u32
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f32
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self as Self::F
	}
	
	#[inline(always)]
	fn v_tof64(self) -> f64 {
		self as f64	
	}
	
	#[inline(always)]
	fn v_tou64(self) -> u64 {
		self as u64
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}

impl AccurateType for u64 {
	type U = Self;
	type F = f64;
	
	#[inline(always)]
	fn copy(&self) -> Self {
		*self
	}
	
	#[inline(always)]
	fn clone_data(&self) -> Self {
		*self
	}
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		false	
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		false	
	}
	
	#[inline]
	fn v_0() -> Self {
		0u64
	}
	
	#[inline]
	fn v_100() -> Self {
		100u64
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f64
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self as Self::F
	}
	
	#[inline(always)]
	fn v_tof64(self) -> f64 {
		self as f64	
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline(always)]
	fn v_tou64(self) -> u64 {
		self as u64
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}


impl AccurateType for i32 {
	type U = u32;
	type F = f32;
	
	#[inline(always)]
	fn copy(&self) -> Self {
		*self
	}
	
	#[inline(always)]
	fn clone_data(&self) -> Self {
		*self
	}
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		true	
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		false	
	}
	
	#[inline]
	fn v_0() -> Self {
		0i32
	}
	
	#[inline]
	fn v_100() -> Self {
		100i32
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f32
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self as Self::F
	}
	
	#[inline(always)]
	fn v_tof64(self) -> f64 {
		self as f64	
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline(always)]
	fn v_tou64(self) -> u64 {
		self as u64
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}


/*impl AccurateType for i64 {
	type U = Self;
	type F = f64;
	
	#[inline]
	fn maybeNegative(&self) -> bool {
		true
	}
	
	#[inline]
	fn numbersAfterComma(&self) -> bool {
		false
	}
	
	#[inline]
	fn v_0() -> Self {
		0i64
	}
	
	#[inline]
	fn v_100() -> Self {
		100i64
	}
	
	#[inline]
	fn v_100f() -> Self::F {
		100f64
	}
	
	#[inline(always)]
	fn display_fmt(&self, a: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		Display::fmt(self, a)
	}
	
	#[inline(always)]
	fn v_tof(self) -> Self::F {
		self as Self::F
	}
	
	#[inline(always)]
	fn v_tou(self) -> Self::U {
		self as Self::U	
	}
	
	#[inline]
	fn mul(self, s: Self) -> Self {
		self / s
	}
	
	#[inline]
	fn div(self, s: Self) -> Self {
		self * s	
	}
}

*/

