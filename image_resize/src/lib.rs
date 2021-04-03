extern crate base64;
extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;

use image::{self, imageops::FilterType, GenericImageView};
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
//pub fn resize(b: String, width: u32, height: u32) -> String {
pub fn resize(buf: Vec<u8>, width: u32, height: u32) -> String {
    console_error_panic_hook::set_once();

    log!("start wasm function");

    //log!("start base64 decode");
    //let buf = base64::decode(b).expect("base64 decode error");
    //log!("end base64 decode");

    log!("start buffer load");
    //let img = image::load_from_memory_with_format(&buf, image::ImageFormat::Jpeg)
    let img = image::load_from_memory(&buf)
        .expect("buffer load error");
    log!("end buffer load");

    let old_w = img.width() as u32;
    let old_h = img.height() as u32;
    log!("old size: {} x {}", old_w, old_h);

    log!("start image resize");
    let resize_image = img.resize_to_fill(width, height, FilterType::Triangle);
    //let resize_image = img.resize_to_fill(width, height, FilterType::CatmullRom);
    //let resize_image = img.resize_to_fill(width, height, FilterType::Lanczos3);
    log!("end image resize");

    let new_w = resize_image.width() as u32;
    let new_h = resize_image.height() as u32;
    log!("new size: {} x {}", new_w, new_h);

    log!("start image write to buffer");
    let mut result: Vec<u8> = Vec::new();
    match resize_image.write_to(&mut result, image::ImageOutputFormat::Jpeg(90)) {
        Ok(_) => log!("buffer write sucess"),
        Err(err) => log!("buffer write error: {}", err),
    }
    log!("end image write to buffer");

    log!("end wasm function");

    return base64::encode(&result);
}
