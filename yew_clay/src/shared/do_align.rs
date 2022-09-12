use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Node};

#[wasm_bindgen(module = "/node_modules/dom-align/dist-web/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = doAlign)]
    fn do_align(el: Element, node_ref: Node) -> Object;
}

// import domAlign from 'dom-align';

// return {
//   points: points,
//   offset: offset,
//   targetOffset: targetOffset,
//   overflow: newOverflowCfg
// }

#[derive(Debug)]
enum Point {
    TopRight,
    ButtomLeft,
}

#[derive(Debug)]
struct AlignBase {
    offset: Option<[u8; 2]>,
    overflow: Option<Overflow>,
    points: Option<[Point; 2]>,
    target_offset: Option<[Point; 2]>,
}

impl From<Object> for AlignBase {
    fn from(js_object: Object) -> Self {
        js_object.into_serde();
        Self {
            offset: todo!(),
            overflow: todo!(),
            points: todo!(),
            target_offset: todo!(),
        }
    }
}

#[derive(Debug)]
struct AlignProps {
    offset: Option<[u8; 2]>,
    overflow: Option<Overflow>,
    points: Option<[Point; 2]>,
    target_offset: Option<[Point; 2]>,
    source_element: Element,
    target_element: Element,
}

#[derive(Debug, Default)]
struct Overflow {
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

// type AlignBase = {
// 	offset?: readonly [number, number];
// 	overflow?: {adjustX: boolean; adjustY: boolean; alwaysByViewport?: boolean};
// 	points?: readonly [string, string];
// 	targetOffset?: readonly [string, string];
// };

// type AlignProps<T, K> = AlignBase & {
// 	sourceElement: K;
// 	targetElement: T;
// };

// function isRtl<T extends HTMLElement>(element: T) {
// 	return window.getComputedStyle(element).direction === 'rtl';
// }

fn do_align(source_element: Element, target_element: Element) -> AlignProps {}

// export function doAlign<T extends HTMLElement, K extends HTMLElement>({
// 	sourceElement,
// 	targetElement,
// 	...config
// }: AlignProps<T, K>): Required<AlignBase> {
// 	return domAlign(sourceElement, targetElement, {
// 		...config,
// 		useCssRight: isRtl(sourceElement),
// 	});
// }
