#![allow(non_snake_case)]

use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use gloo::utils::window;
use gloo_timers::future::TimeoutFuture;
use grid::Grid;
use model::{lenia::Lenia, Model};
use tracing::Level;
use web_sys::wasm_bindgen::{Clamped, JsCast};

mod error;
mod grid;
mod model;

pub use error::Error;

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

pub enum Action {
    Start,
    Stop,
    Pause,
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    let mut state = use_signal(|| Grid::new(WIDTH, HEIGHT));
    let mut count = use_signal(|| 0);
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
                let value = (255.0 * state.at(x, y)) as u8;
                dst_pixel[0..=2].copy_from_slice(&[value, value, value]);
                dst_pixel[3] = 255;
            }
        }

        let image_data = web_sys::ImageData::new_with_u8_clamped_array(Clamped(data.as_slice()), width as u32).unwrap();
        context.put_image_data(&image_data, 0., 0.).unwrap();
    });

    let lenia = Lenia::new(7, 1.0);
    let tx = use_coroutine(move |mut rx: UnboundedReceiver<Action>| async move {
        loop {
            if let Some(Action::Start) = rx.next().await {
                'inner: loop {
                    TimeoutFuture::new(100).await;
                    let _ = state.write().update(lenia.kernel());
                    count.set(count + 1);
                    match rx.try_next() {
                        Ok(Some(Action::Pause)) => break 'inner,
                        Ok(Some(Action::Stop)) => {
                            count.set(0);
                            break 'inner;
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    rsx! {
        div {
            style: "text-align: center;",
            canvas {
                id: "pixels",
                style: "border: 1px solid rgb(110,220,230); width: {WIDTH}px; height: {HEIGHT}px;",
                width: "{WIDTH}",
                height: "{HEIGHT}",
            },
            p { "Timestep: {count}" },
            button { onclick: move |_| tx.send(Action::Start), "Start" },
            button { onclick: move |_| tx.send(Action::Stop), "Stop" },
            button { onclick: move |_| tx.send(Action::Pause), "Pause" },
        }
    }
}
