use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Element;

#[wasm_bindgen(module = "dom_align/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = doAlign)]
    fn do_align(source_node: &Element, target_node: &Element, config: JsValue) -> JsValue;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Point {
    TopRight,
    ButtomLeft,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AlignConfig {
    offset: Option<[u8; 2]>,
    overflow: Option<Overflow>,
    points: Option<[Point; 2]>,
    target_offset: Option<[Point; 2]>,
    use_css_right: bool,
}

impl AlignConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_offset(&mut self, offset: [u8; 2]) {
        self.offset = Some(offset)
    }

    pub fn set_overflow(&mut self, overflow: Overflow) {
        self.overflow = Some(overflow)
    }

    pub fn set_points(&mut self, points: [Point; 2]) {
        self.points = Some(points)
    }

    pub fn set_target_offset(&mut self, points: [Point; 2]) {
        self.target_offset = Some(points)
    }

    pub fn set_use_css_right(&mut self, use_css_right: bool) {
        self.use_css_right = use_css_right
    }
}

impl TryFrom<JsValue> for AlignConfig {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let value: Self = serde_wasm_bindgen::from_value(value)?;

        Ok(value)
    }
}

impl From<AlignConfig> for JsValue {
    fn from(_: AlignConfig) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct AlignProps<'a> {
    offset: Option<[u8; 2]>,
    overflow: Option<Overflow>,
    points: Option<[Point; 2]>,
    target_offset: Option<[Point; 2]>,
    source_element: &'a Element,
    target_element: &'a Element,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Overflow {
    adjust_x: bool,
    adjust_y: bool,
    always_by_viewport: bool,
}

fn is_rtl(element: &Element) -> bool {
    let window = web_sys::window().expect("there to be a window");
    let computed_style = window
        .get_computed_style(element)
        .expect("get_computed_style not to fail");
    if let Some(computed_style) = computed_style {
        let direction = computed_style
            .get_property_value("direction")
            .expect("get_property_value not to fail");

        direction == "rtl"
    } else {
        true
    }
}

pub fn align_dom<'a>(
    source_element: &'a Element,
    target_element: &'a Element,
    mut config: AlignConfig,
) -> AlignProps<'a> {
    config.set_use_css_right(is_rtl(source_element));

    let config =
        serde_wasm_bindgen::to_value(&config).expect("config to be convertable to js value.");

    let result = do_align(source_element, target_element, config);

    let as_rust = AlignConfig::try_from(result).expect("it to be possible to convert the result.");

    AlignProps {
        offset: as_rust.offset,
        overflow: as_rust.overflow,
        points: as_rust.points,
        target_offset: as_rust.target_offset,
        source_element,
        target_element,
    }
}
