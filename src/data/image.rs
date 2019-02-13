use super::*;

struct ImageType;

impl DataType for ImageType {
    fn color() -> String {
        "red".to_owned()
    }
}

struct Image<Depth> {
    channels: Vec<String>,
    height: usize,
    width: usize,
    data: Vec<Depth>
}

impl<Depth> Data for Image<Depth> {
    fn default_options() -> Vec<viewer::ViewerOption> {
        vec![viewer::ViewerOption::Channel(vec!["R".to_owned(), "G".to_owned(), "B".to_owned(), "A".to_owned()])]
    }

    fn display(&self, options: Vec<viewer::ViewerOption>, builder: winit::WindowBuilder) -> winit::WindowBuilder {
        builder
    }
}