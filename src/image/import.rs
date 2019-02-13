use super::*;

pub struct Import;

impl Node for Import {
    fn name() -> String { "Import".to_owned() }
    fn input_names() -> Vec<DotInfo> { Vec::new() }
    fn output_names() -> Vec<DotInfo> { vec![("Output".to_owned(), image::ImageType)] }


}