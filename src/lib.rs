use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

use naga::ShaderStage;

use naga::front::glsl::{Parser, Options};
use naga::back::wgsl;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn glsl2wgsl(glsl: &str, ext: &str) -> String {
    console_error_panic_hook::set_once();
    let mut parser = Parser::default();
    let stage = match ext {
        "vert" => ShaderStage::Vertex,
        "frag" => ShaderStage::Fragment,
        "comp" => ShaderStage::Compute,
        _ => unreachable!(),
    };
    let options = Options::from( stage );

    let result = parser.parse(&options, glsl);

    let module = match result {
        Ok(module) => module,
        Err(error) => panic!("Error: {:?}", error),
    };

    let info_result = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::empty()).validate(&module);
    let info = match info_result {
        Ok(info) => info,
        Err(error) => panic!("Error: {:?}", error),
    };

    let wgsl_result = wgsl::write_string(
        &module,
        &info,
        wgsl::WriterFlags::empty(),
    );

    let wgsl = match wgsl_result {
        Ok(wgsl) => wgsl,
        Err(error) => panic!("Error: {:?}", error),
    };

    return wgsl
}