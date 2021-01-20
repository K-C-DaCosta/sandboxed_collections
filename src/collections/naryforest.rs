pub type Pointer = u32;
pub static NULL: Pointer = !0;

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

    /// # Description
    /// Searches through all trees in the forest and returns a list of pointers that
    /// satify `predicate`.
    /// # Parameters
    /// - predicate - use this to specify search criteria
    /// - max_results - specify maximum number of results we wish to collect
    /// # Returns
    /// A vec of pointers satifying `predicate`
    pub fn search_all<CB>(&self, predicate: CB, max_results: usize) -> Vec<Pointer>
    where
        CB: Fn(&NaryNode<T>) -> bool + Copy,
    {
        let mut results = Vec::new();
        for &root_ptr in self.root_list.iter() {
            self.search_and_collect(predicate, root_ptr, &mut results, max_results)
        }
        results
    }

    /// # Description
    /// Same as `search_all(..)` but now search is from an arbitrary `root`
    pub fn search_and_collect<CB>(
        &self,
        predicate: CB,
        root: Pointer,
        results: &mut Vec<Pointer>,
        max_results: usize,
    ) where
        CB: Fn(&NaryNode<T>) -> bool + Copy,
    {
        if root == NULL || results.len() >= max_results {
            return;
        }
        if predicate(&self[root]) {
            results.push(root);
        }
        for &child_ptr in self[root].children.iter() {
            self.search_and_collect(predicate, child_ptr, results, max_results);
        }
    }

    /// # Description
    /// Searches from a `root` and returns pointer to the first item that satifyies `predicate`
    pub fn search<CB>(&self, predicate: CB, root: Pointer) -> Option<Pointer>
    where
        CB: Fn(&NaryNode<T>) -> bool + Copy,
    {
        if root == NULL {
            return None;
        }

        if predicate(&self[root]) {
            return Some(root);
        }

        for &child_ptr in self[root].children.iter() {
            let res = self.search(predicate, child_ptr);
            if res.is_some() {
                return res;
            }
        }

        None
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
