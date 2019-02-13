extern crate failure;
#[macro_use] extern crate failure_derive;

extern crate uuid;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rayon;

extern crate sciter;
extern crate sciter_serde;

extern crate vulkano;
extern crate winit;

pub use std::fs;
pub use fs::File;
pub use std::thread;
pub use std::path::PathBuf;
pub use std::sync::{Arc, Mutex, atomic};
pub use std::collections::HashMap;

pub use uuid::Uuid;

pub use sciter::{host, window::Options, utf::w2s, Value, HELEMENT, EventHandler, HostHandler};

pub mod model;
use model::*;

pub mod handler;
pub mod data;
pub mod channel;
pub mod graph;

//nodes
pub mod image;

#[cfg(not(debug_assertions))]
pub struct MainHostHandler {
    assets: sciter::Archive
}

#[cfg(not(debug_assertions))]
impl HostHandler for MainHostHandler {
    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri = &w2s(pnm.uri);
        let filepath: PathBuf = uri["file://".to_string().len()..].to_owned().into();

        self.assets.get(&filepath.to_string_lossy()).map(|data| {
            self.data_ready(pnm.hwnd, &uri, data, None);
            host::LOAD_RESULT::LOAD_DEFAULT
        })
    }
}

#[cfg(debug_assertions)]
struct DebugHostHandler;

#[cfg(debug_assertions)]
impl HostHandler for DebugHostHandler {
    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri = &w2s(pnm.uri);

        let mut path = std::env::current_dir().unwrap();
        path.push("ui");

        let filepath: PathBuf = uri["file://".to_string().len()..].to_owned().into();

        path.push(filepath);
        if let Ok(b) = fs::read(path) {
            self.data_ready(pnm.hwnd, &uri, &b.as_slice(), None);
            return Some(host::LOAD_RESULT::LOAD_DEFAULT);
        }

        None
    }
}

#[cfg(debug_assertions)]
fn setup(win: &mut sciter::Window) {
    win.sciter_handler(DebugHostHandler);
    win.load_file("file://main.html");
}

#[cfg(not(debug_assertions))]
fn setup(win: &mut sciter::Window) {
    let archive = include_bytes!("./resources.rc");
    let assets = host::Archive::open(archive).expect("Error loading archive!");

    win.sciter_handler(MainHostHandler {assets});
    win.load_file("file://main.html");
}

fn start_win(mstate: MState) -> sciter::Window {
    let mut win = sciter::WindowBuilder::main_window().resizeable().alpha().glassy().with_size((900, 800)).create();
    let opts = vec![Options::TransparentWindow(true), Options::AlphaWindow(true), Options::DebugMode(cfg!(debug_assertions))];

    opts.iter().for_each(|opt| win.set_options(opt.clone()).expect("Error setting debugmode!"));

    win.set_title("Dreamer");

    win.event_handler(handler::Handler {state: mstate.clone()});
    setup(&mut win);

    let sv = sciter_serde::to_value(&*mstate.lock().unwrap()).unwrap();
    let _ = win.get_host().call_function("load", &[sv]);
    win
}

fn main() {
    let state = State::new();
    let mstate = Arc::new(Mutex::new(state));

    let win = start_win(mstate.clone());
    win.run_app();
}
