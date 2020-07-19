extern crate image_convert;

use image_convert::{to_webp, ImageResource, WEBPConfig};
use neon::prelude::*;
use neon::register_module;
use std::path::Path;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    let name = format!("{}{}", cx.argument::<JsString>(1)?.value(), ".webp");

    let output_path = format!("{}{}", "./output/", name);

    println!("path is:{} name:{} output_path:{}", path, name, output_path);

    let source_image_path = Path::new(&path);
    let output_image_path = Path::new(&output_path);

    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(output_image_path);

    let mut config = WEBPConfig::new();
    config.width = 800;
    config.quality = 100;

    to_webp(&mut output, &input, &config).unwrap();

    Ok(cx.string(name))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
