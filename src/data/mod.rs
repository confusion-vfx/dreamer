use super::*;
pub mod viewer;
pub mod image;

pub trait DataType {
    fn color() -> String;
}

pub trait Data {
    fn default_options() -> Vec<viewer::ViewerOption> where Self: Sized;

    fn display(&self, options: Vec<viewer::ViewerOption>, builder: winit::WindowBuilder) -> winit::WindowBuilder;
}