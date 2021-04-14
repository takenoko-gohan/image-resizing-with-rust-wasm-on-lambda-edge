extern crate base64;
extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;

use image::{self, imageops::FilterType};
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn resize(buf: Vec<u8>, width: u32, height: u32, format: &str) -> String {
    console_error_panic_hook::set_once();

    log!("start wasm function");

    // nodejs から渡されたバッファーを読み込む
    log!("start buffer load");
    let img = image::load_from_memory(&buf).expect("buffer load error");
    log!("end buffer load");

    // リサイズ
    log!("start image resize");
    let resize_image = img.resize_to_fill(width, height, FilterType::Triangle);
    log!("end image resize");

    // リサイズ結果を書き込む
    log!("start image write to buffer");
    let mut result: Vec<u8> = Vec::new();

    match format {
        "jpeg" | "jpg" => {
            log!("match jpeg");
            match resize_image.write_to(&mut result, image::ImageOutputFormat::Jpeg(90)) {
                Ok(_) => log!("buffer write sucess"),
                Err(err) => log!("buffer write error: {}", err),
            }
        },
        "png" => {
            log!("match png");
            match resize_image.write_to(&mut result, image::ImageOutputFormat::Png) {
                Ok(_) => log!("buffer write sucess"),
                Err(err) => log!("buffer write error: {}", err),
            }
        },
        _ => {
            log!("did not match");
        },
    }
    log!("end image write to buffer");

    log!("end wasm function");

    // BASE64 で返す
    return base64::encode(&result);
}
