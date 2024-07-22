#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo::utils::window;
use ndarray::Array2;
use tracing::Level;
use web_sys::wasm_bindgen::{Clamped, JsCast};

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    let state = use_signal(|| Array2::<f64>::zeros((WIDTH, HEIGHT)));

    use_effect(move || {
        let document = window().document();
        let canvas = document.unwrap().get_element_by_id("pixels").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let width = canvas
            .get_attribute("width")
            .map(|w| w.parse::<usize>().unwrap_or_default())
            .unwrap_or_default()
            .min(WIDTH);
        let height = canvas
            .get_attribute("height")
            .map(|h| h.parse::<usize>().unwrap_or_default())
            .unwrap_or_default()
            .min(HEIGHT);
        let image_data = context
            .create_image_data_with_sw_and_sh(width as f64, height as f64)
            .unwrap();
        let bytes_per_line = width * 4;
        let mut data = image_data.data();

        let state = state.read();
        for y in 0..height {
            for x in 0..width {
                let offset = (y * bytes_per_line + 4 * x) as usize;
                let dst_pixel = &mut data[offset..offset + 4];
                let value = (255.0 * state[[x, y]]) as u8;
                dst_pixel[0..=2].copy_from_slice(&[value, value, value]);
                dst_pixel[3] = 255;
            }
        }

        let image_data = web_sys::ImageData::new_with_u8_clamped_array(Clamped(data.as_slice()), width as u32).unwrap();
        context.put_image_data(&image_data, 0., 0.).unwrap();
    });

    rsx! {
        div {
            style: "text-align: center;",
            link { rel: "stylesheet", href: "main.css" }
            canvas {
                id: "pixels",
                style: "border: 1px solid rgb(110,220,230); width: {WIDTH}px; height: {HEIGHT}px;",
                width: "{WIDTH}",
                height: "{HEIGHT}",
            },
        }
    }
}
