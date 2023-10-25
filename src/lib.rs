extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn transform(input: f64, input_range_start: f64, input_range_end: f64, output_range_start: f64, output_range_end: f64) -> f64 {
    let input_span = input_range_end - input_range_start;
    let output_span = output_range_end - output_range_start;
    let normalized_input = (input - input_range_start) / input_span;
    output_range_start + normalized_input * output_span
}
