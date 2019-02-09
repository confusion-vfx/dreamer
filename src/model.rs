use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {

}

pub type MState = Arc<Mutex<State>>;

impl State {
    pub fn new() -> Self {
        State { }
    }
}