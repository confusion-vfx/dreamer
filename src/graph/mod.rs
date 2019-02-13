use super::*;

#[derive(Debug, Fail)]
pub enum NodeError {
    #[fail(display = "Vulkan Error: {}", 0)]
    GpuError(String)
}

pub type NodeRes<T> = Result<T, NodeError>;

pub enum ProcessState {
    Processing, Processed
}

pub type Input = Arc<Mutex<NodeContext<dyn Node>>>;
pub type DotInfo = (String, Box<dyn data::DataType>);

pub struct NodeContext<T: Node + ?Sized> {
    pub node: Box<T>,
    pub name: String,

    pub input_names: Vec<DotInfo>,
    pub input: Vec<Input>,

    pub output_names: Vec<DotInfo>,
    pub output: Vec<Box<dyn data::Data>>,

    pub channels: HashMap<String, channel::Channel>
}

pub struct ProcessContext<T: Node> {
    pub ctx: Arc<Mutex<NodeContext<T>>>,
    pub cancel: atomic::AtomicBool,
    // vulkan context
}

pub trait Node {
    fn name() -> String where Self: Sized;
    fn input_names() -> Vec<DotInfo> where Self: Sized;
    fn output_names() -> Vec<DotInfo> where Self: Sized { vec![("Output".to_owned(), data::image::Image)] }
    fn channels() -> HashMap<String, channel::Channel> where Self: Sized;

    fn initialize_ctx(self) -> NodeContext<Self> where Self: Sized {
        NodeContext {
            node: Box::new(self),
            name: Self::name(),
            input_names: Self::input_names(),
            input: Vec::new(),
            output_names: Self::output_names(),
            output: Vec::new(),
            channels: Self::channels()
        }
    }

    fn connect(ctx: &Mutex<NodeContext<Self>>, i: usize, inp: Input) -> NodeRes<()> where Self: Sized {
        let mut l = ctx.lock().unwrap();
        l.input.insert(i, inp);
        Ok(())
    }

    fn disconnect(ctx: &Mutex<NodeContext<Self>>, i: usize) -> NodeRes<()> where Self: Sized {
        ctx.lock().unwrap().input.remove(i); Ok(())
    }

    fn process(ctx: &ProcessContext<Self>) -> NodeRes<()> where Self: Sized;

    fn time_process(ctx: &ProcessContext<Self>) -> NodeRes<()> where Self: Sized;
}

pub struct NodeInfo<T: Node> {
    initialize: Fn() -> T,
    category: String,
    name: String
}

pub type NodeId = Uuid;

pub struct NodeGraph {
    nodes: HashMap<NodeId, Arc<Mutex<NodeContext<dyn Node>>>>,
    toolset: Vec<NodeInfo<dyn Node>>
}

impl NodeGraph {
    fn new() -> Self {
        NodeGraph {
            nodes: HashMap::new(),
            toolset: Vec::new()
        }
    }

    fn add_node<T: Node>(&mut self, node: Vec<NodeInfo<T>>) {
        self.toolset.append(Box::new(node));
    }
}