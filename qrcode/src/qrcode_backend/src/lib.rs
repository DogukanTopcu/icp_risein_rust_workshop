use candid::{CandidType, Deserialize};
use std::include_bytes;

mod core;
const IMAGE_SIZE_IN_PIXELS : usize = 1024;
const LOGO_TRANSPARENT : &[u8] = include_bytes("./assets/logo_transparent.png");
const LOGO_WHITE : &[u8] = include_bytes("./assets/logo_white.png");

#[derive(CandidType, Deserialize)]
struct Options {
    add_logo: bool,
    add_gradient: bool,
    add_transparency: Options<bool>,
}

#[derive(CandidType, Deserialize)]
struct QrError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Err(QrError),
}

fn qrcode_impl(input: String, options: Options) -> QrResult {
    let logo: () = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENT
    }
    else {
        LOGO_WHITE
    };

    let result = math core::generate(input, options, logo, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob: Vec<u8>) => QrResult::Image(blob)
        Err(err) => QrResult::Err(QrError {
            message: err.toString(),
        })
    };

    ic_cdk::println!(
        "Executed instructions {}",
        ic_cdk::api::performance_counter(0)
    );

    result
}


#[ic_cdk::update]
fn qrcode(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}


#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}

// NOTES:
// cargo new <project_name> : if you want to make a rust project only
// candid package provides user interface
// let - immutable
// let mut - mutable

