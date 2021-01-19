use std::ops;

// The pointer type for tree memory
pub type NodeAddr = u64;
pub static NULL: NodeAddr = !0;

pub struct NaryNode<T> {
    pub parent: NodeAddr,
    pub data: Option<T>,
    pub children: Vec<NodeAddr>,
}
impl<T> NaryNode<T> {
    pub fn add_child(&mut self, child_addr: NodeAddr, parent_addr: NodeAddr) {
        self.children.push(child_addr);
        self.parent = parent_addr;
    }

    pub fn nullify(&mut self) {
        self.parent = NULL;
        self.data = None;
        self.children.clear();
    }
}

impl<T> From<Option<T>> for NaryNode<T> {
    fn from(data: Option<T>) -> Self {
        Self {
            parent: NULL,
            data,
            children: Vec::new(),
        }
    }
}

pub struct NaryTree<T> {
    pub root: NodeAddr,
    pub memory: Vec<NaryNode<T>>,
    pub node_pool: Vec<NodeAddr>,
}

impl<T> NaryTree<T> {
    /// # Description
    /// -initalizes a N-ary tree
    /// -new() does no allocation
    pub fn new() -> Self {
        Self {
            root: NULL,
            memory: Vec::new(),
            node_pool: Vec::new(),
        }
    }

    /// # Descrption
    /// allocates node
    /// # Returns
    /// the address of the newly allocated node
    pub fn allocate_node(&mut self, data: Option<T>) -> NodeAddr {
        match self.node_pool.pop() {
            Some(node_ptr) => {
                self[node_ptr].nullify();
                node_ptr
            }
            None => {
                let node = NaryNode::from(data);
                self.memory.push(node);
                self.memory.len() as NodeAddr - 1
            }
        }
    }

    /// # Description
    /// 'frees' a node at address `node-ref`
    /// # Comments
    /// The function doesn't actually free a node but instead caches it.
    /// When allocate(..) is called the freed node will be used again.
    pub fn free_node(&mut self, node_ref: NodeAddr) {
        if node_ref != NULL {
            self[node_ref].data.take();
            self.node_pool.push(node_ref);
        }
    }

    /// # Description
    /// Clears entire tree in O(1)
    pub fn clear(&mut self) {
        self.root = NULL;
        self.memory.clear();
        self.node_pool.clear();
    }
}

impl<T> ops::Index<NodeAddr> for NaryTree<T> {
    type Output = NaryNode<T>;
    fn index(&self, index: NodeAddr) -> &Self::Output {
        &self.memory[index as usize]
    }
}

impl<T> ops::IndexMut<NodeAddr> for NaryTree<T> {
    fn index_mut(&mut self, index: NodeAddr) -> &mut Self::Output {
        &mut self.memory[index as usize]
    }
}
