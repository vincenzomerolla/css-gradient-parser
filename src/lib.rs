use lightningcss::properties::background::Background;
use lightningcss::traits::Parse;
use lightningcss::values::image;
use neon::prelude::*;
use serde_json::to_string;

fn parse_to_json_str(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg: Handle<JsString> = cx.argument(0)?;
    let val = arg.value(&mut cx);
    let bg = Background::parse_string(val.as_str()).unwrap();
    match bg.image {
        image::Image::Gradient(g) => {
            let gradient = to_string(&g).unwrap();
            Ok(cx.string(gradient))
        }
        _ => Ok(cx.string("Not a gradient")),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse_to_json_str", parse_to_json_str)?;
    Ok(())
}
