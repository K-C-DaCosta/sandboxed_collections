type Pointer = u32;
static NULL: Pointer = !0;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct NaryNode<T> {
    pub parent: Pointer,
    pub data: Option<T>,
    pub children: Vec<Pointer>,
}

impl<T> NaryNode<T> {
    pub fn new() -> NaryNode<T> {
        NaryNode {
            parent: NULL,
            data: None,
            children: Vec::new(),
        }
    }
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}

///Custom N-ary Tree implemented with vector-backed memory
#[derive(Clone)]
pub struct NaryForest<T> {
    pub root_list: Vec<Pointer>,
    pub pool: Pointer,
    pub memory: Vec<NaryNode<T>>,
}

impl<T> NaryForest<T>
where
    Self: Index<u32, Output = NaryNode<T>> + IndexMut<u32>,
{
    pub fn new() -> NaryForest<T> {
        NaryForest {
            root_list: Vec::new(),
            pool: NULL,
            memory: Vec::new(),
        }
    }
    pub fn allocate(&mut self, val: T) -> Pointer {
        if self.pool == NULL {
            self.memory.push(NaryNode::new().with_data(val));
            (self.memory.len() - 1) as u32
        } else {
            let pool_node = self.pool;
            self.pool = self[pool_node].children[0];
            self[pool_node].children.clear();
            pool_node
        }
    }
    
    #[allow(dead_code)]
    pub fn free(&mut self, node: Pointer) {
        if node == NULL {
            return;
        }
        if self.pool != NULL {
            let old_pool = self.pool;
            self[node].children.clear();
            self[node].children.push(old_pool);
        }
        self.pool = node;
    }

    #[allow(dead_code)]
    pub fn allocate_node(&mut self, node: NaryNode<T>) -> Pointer {
        if self.pool == NULL {
            self.memory.push(node);
            (self.memory.len() - 1) as u32
        } else {
            let pool_node = self.pool;
            self.pool = self[pool_node].children[0];
            self[pool_node].children.clear();
            pool_node
        }
    }
    pub fn add_child(&mut self, parent: Pointer, child: Pointer) {
        self[parent].children.push(child);
        self[child].parent = parent;
    }
}

impl<T> Index<u32> for NaryForest<T> {
    type Output = NaryNode<T>;

    fn index(&self, ptr: u32) -> &Self::Output {
        self.memory.get(ptr as usize).unwrap()
    }
}

impl<T> IndexMut<u32> for NaryForest<T> {
    fn index_mut(&mut self, ptr: u32) -> &mut Self::Output {
        self.memory.get_mut(ptr as usize).unwrap()
    }
}
