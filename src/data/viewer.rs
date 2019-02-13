use super::*;

struct Viewer {
    pub node_viewing: Option<graph::Input>
}

pub enum ViewerOption {
    Channel(Vec<String>)
}