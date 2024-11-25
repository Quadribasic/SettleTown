#[derive(Resource)]
pub struct LayerStack {
    /// A stack of layers to be rendered, the lower ones pushed on first
    stack: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn add_layer(&mut self, to_add: Layer) {
        self.stack.push(to_add);
    }
}
