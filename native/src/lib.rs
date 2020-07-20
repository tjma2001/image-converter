extern crate image_convert;

use image_convert::{to_webp, ImageResource, WEBPConfig};
use neon::prelude::*;
use neon::register_module;
use std::path::Path;

fn convert(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    let name = format!("{}{}", cx.argument::<JsString>(1)?.value(), ".webp");
    let width = cx.argument::<JsNumber>(2)?.value() as u16;

    let output_path = format!("{}{}", "./output/", name);

    let source_image_path = Path::new(&path);
    let output_image_path = Path::new(&output_path);

    let input = ImageResource::from_path(source_image_path);
    let mut output = ImageResource::from_path(output_image_path);

    let mut config = WEBPConfig::new();

    if width > 0 {
        config.width = width;
    }

    config.quality = 100;

    to_webp(&mut output, &input, &config).unwrap();

    Ok(cx.string(name))
}

register_module!(mut cx, { cx.export_function("convert", convert) });
