- ui
  - node search
  - viewer zoom
  - node panning
- back
  - node store
    - node id
    - node name, data
  - add node
  - remove node
  - modify node
  - load viewer
  - open comp -> nodes
  - close comp -> nodes
  - viewers linked to node ids
- ideas
  - wireless nodes are like elbow nodes with different color
  - liveoutput node that uses gstreamer
- what if wires are nodes

- datatype trait
  - type data
  - display (data, hwnd or something, options) -> datares
    - gfx or another renderer
  - zoom
  - pan
  - rotate
  - select
  - options
    - 3d & vr
  - color
  - name
  - subviews todo

- channel trait
  - 

- node trait
  - inputs -> vec<String, DataType>
  - outputs -> vec<String, DataType>
  - channels -> vec<string, channeltype>
  - process (hashmap, hashmap) -> noderes<hashmap>
  - time_change -> cacheresult enum (whether to clearcache or not)
  - frompython or something for both traits

what we need is: 
a schema of the types
parameters having same type to validate/annex
do not query data if not needed

so process should take data
and data should be same type of
- hashmap
  - string for name from inputs
  - and the data being a node
  - node gives data
    - node cannot have type parameters or be trait

# Channels
## Requirements
- have different types
- be a nice enum



# Nodes
## Requirements
- have process state of processing, processed
- have state with inputs, main output, outputs and channels
- be able to set initial state and state on hooks
- update based on context (holding cancellation info), input nodes, channels and state
  - take mut self and return processtate
- time change update separate
- connect/disconnect based on node graph and node to connect

```rust
trait Node {
  fn inputs(&self) -> HashMap<String, DataType>;
  fn outputs(&self) -> HashMap<String, DataType>;
  
  fn channels(&self) -> HashMap<String, ChannelType>;
  
  fn process(ctx: NodeContext<Self>, inputs: HashMap<String, Box<dyn Input>>, channels: HashMap<String, Box<dyn Channel>>) -> ProcessState;
}
```

## ProcessState

```rust
enum ProcessState {
  Processing, Processed(dyn Data)
}
```

## Context

```rust
struct NodeContext<T: Node> {
  node: Arc<Mutex<T>>,
  cancel: AtomicBool,
  vulkano context
}

impl NodeContext {
  fn run_shader(&mut self, buffer: CpuAccessibleBuffer) -> NodeRes<CpuAccessibleBuffer>
}
```

## Inputs

```rust
struct Input {
  node: Arc<Mutex<Node>>
}


```

# NodeGraph
## Requirements
- know when channel/inputs change
  - clear node from queue
  - check viewer dependencies
  - queue solve
  - update viewers
- time change
  - same thing but with time change

```rust
type NodeId = Uuid;

struct NodeGraph {
  nodes: HashMap<NodeId, Arc<Mutex<Box<dyn Node>>>>
}

impl NodeGraph {
  fn insert_node(node: Node) -> NodeId;
  fn depends_on(&self, node_id: NodeId) -> Vec<&Node>;
}
```

# Viewers
## Requirements
- have access to what node they are optionally viewing


https://crates.rs/crates/easer

- image trait
  - channels : vec name
  - height
  - width
  - data: time -> vec height * width * channels

- processing workflow for affine
  - image loads image into image
  - 