
extern crate cluMeasurementData;

use cluMeasurementData::web_values::DataVH;
use cluMeasurementData::web_values::ToVH;
use cluMeasurementData::screen::ScreenResolution;
use cluMeasurementData::screen::ToElementResolution;
use cluMeasurementData::values::ToPx;

fn main() {
	
	let resolution = ScreenResolution::resolution(1280_u32, 699_u32);
	
	let vh: DataVH<f64> = 699_u32.to_element_resolution().to_vh(12u32.to_px());
	let vhu: DataVH<u64> = 699_u32.to_element_resolution().to_vh(12u32.to_px());
	
	println!("{}", vh);
	println!("{}", vhu);
	
	let vh_resolution: DataVH<f64> = resolution.new_vh(12u32.to_px());
	println!("{}", vh_resolution);
}
