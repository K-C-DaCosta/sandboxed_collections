pub const FRONT: usize = 0;
pub const REAR: usize = 1;

use std::{ops, usize};

struct IncrementQuery {
    old_ptr: usize,
    cur_ptr: usize,
}
/// # Descirption
/// A fixed-capacity ring buffer
pub struct RingBuffer<Memory> {
    len: usize,
    capacity: usize,
    pointers: [usize; 2],
    memory: Memory,
}

impl<T> RingBuffer<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            len: 0,
            capacity: 0,
            pointers: [0; 2],
            memory: T::default(),
        }
    }
}

impl<T> RingBuffer<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.pointers = [0; 2];
    }

    pub fn is_empty(&self) -> bool {
        self.len <= 0
    }

    pub fn is_full(&self) -> bool {
        self.len >= self.capacity
    }

    pub fn front(&self) -> usize {
        self.pointers[FRONT]
    }

    pub fn rear(&self) -> usize {
        self.pointers[REAR]
    }

    /// # Description
    /// Makes room for newly enqueued item and retuns location of newly allocated index
    /// # returns
    ///  None if enqueue fails
    pub fn enqueue(&mut self) -> Option<usize> {
        self.increment_pointer(|rb| rb.is_full(), REAR, 1)
            .map(|IncrementQuery { old_ptr, .. }| old_ptr)
    }
    /// # Description
    /// deques item and retuns location of recently dequed item  
    /// # returns
    /// `None` if dequeue fails
    pub fn dequeue(&mut self) -> Option<usize> {
        self.increment_pointer(|rb| rb.is_empty(), FRONT, -1)
            .map(|IncrementQuery { old_ptr, .. }| old_ptr)
    }

    /// # Description
    /// pops the rear and returns the index to the popped item
    /// # Returns
    /// `None` is pop fails
    pub fn pop_rear(&mut self) -> Option<usize> {
        if self.is_empty() {
            None
        } else {
            self.pointers[REAR] = (self.pointers[REAR] + self.capacity - 1) % self.capacity;
            self.len -= 1;
            Some(self.pointers[REAR])
        }
    }

    /// # Description
    /// This generalization of enqueue/dequeue operation
    /// I noticed that both enqueue and dequeue are extremely similar so i merged the operations into one
    /// function here
    fn increment_pointer<CB>(
        &mut self,
        has_no_space: CB,
        pointer_type: usize,
        len_inc_dec: isize,
    ) -> Option<IncrementQuery>
    where
        CB: Fn(&Self) -> bool,
    {
        if has_no_space(self) {
            None
        } else {
            let old_ptr = self.pointers[pointer_type];
            self.pointers[pointer_type] = (self.pointers[pointer_type] + 1) % self.capacity;
            self.len = ((self.len as isize) + len_inc_dec) as usize;
            Some(IncrementQuery {
                old_ptr,
                cur_ptr: self.pointers[pointer_type],
            })
        }
    }

    /// # Description
    /// retuns index into the next item
    pub fn peek_next(&self) -> Option<usize> {
        if self.len <= 1 {
            None
        } else {
            Some((self.pointers[FRONT] + 1) % self.capacity)
        }
    }

    fn index_iter(&self) -> RingIter {
        RingIter {
            cur: self.pointers[FRONT],
            cap: self.capacity,
            len: self.len,
        }
    }
}

impl<T> RingBuffer<Vec<T>>
where
    T: Sized + Default + Clone,
{
    pub fn with_capacity(mut self, cap: usize) -> Self {
        self.capacity = cap;
        self.memory.resize(cap, T::default());
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.index_iter().map(move |i| &self.memory[i])
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.index_iter()
            .map(move |i| unsafe { &mut *self.memory.as_mut_ptr().offset(i as isize) })
    }
}
///# Description
/// Use this enum create and initalize ring buffers to various sizes
pub enum RingSpecifier<Memory> {
    /// # Description
    /// used to create and empty queue with the `Memory`'s length being the capacity
    /// # Comments
    /// basically it enqueues nothing in the vector and the capacity of the ring
    /// comes from the vector's current length
    MakeEmpty(Memory),

    /// # Description
    /// used to create and empty queue with the `Memory`'s length being the capacity.
    /// # Comments
    /// basically it enqueues everything in the `Memory` and the capacity of the ring
    /// comes from the `Memory`'s current length
    MakeFull(Memory),
}

impl<T> From<RingSpecifier<Vec<T>>> for RingBuffer<Vec<T>> {
    fn from(spec: RingSpecifier<Vec<T>>) -> Self {
        match spec {
            RingSpecifier::MakeEmpty(mem) => Self {
                len: 0,
                pointers: [0, 0],
                capacity: mem.len(),
                memory: mem,
            },
            RingSpecifier::MakeFull(mem) => Self {
                len: mem.len(),
                pointers: [0, 0],
                capacity: mem.len(),
                memory: mem,
            },
        }
    }
}

impl<T> ops::Index<Option<usize>> for RingBuffer<Vec<T>> {
    type Output = T;
    fn index(&self, index: Option<usize>) -> &Self::Output {
        index.map(|a| self.memory.get(a)).flatten().unwrap()
    }
}
impl<T> ops::IndexMut<Option<usize>> for RingBuffer<Vec<T>> {
    fn index_mut(&mut self, index: Option<usize>) -> &mut Self::Output {
        index
            .map(move |a| self.memory.get_mut(a))
            .flatten()
            .unwrap()
    }
}

impl<T> ops::Index<usize> for RingBuffer<Vec<T>> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl<T> ops::IndexMut<usize> for RingBuffer<Vec<T>> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

pub struct RingIter {
    cur: usize,
    cap: usize,
    len: usize,
}

impl Iterator for RingIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            let old_cur = self.cur;
            self.len -= 1;
            self.cur = (self.cur + 1) % self.cap;
            Some(old_cur)
        }
    }
}

#[test]
fn ring_buffer_base_cases() {
    let rb: RingBuffer<Vec<i32>> = RingBuffer::new();

    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![]);
    assert_eq!(rb.is_empty(), true);
    assert_eq!(rb.is_full(), true);

    let rb = RingBuffer::from(RingSpecifier::MakeFull(vec![0]));
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![0]);
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), true);
}

#[test]
fn ring_buffer_deq_tests() {
    let mut rb = RingBuffer::from(RingSpecifier::MakeFull(vec![1, 2, 3, 4, 5, 6, 7]));

    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), true);
    assert_eq!(
        rb.iter().map(|&a| a).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7]
    );

    let decd_item = rb.dequeue();
    assert_eq!(rb[decd_item], 1);
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(
        rb.iter().map(|&a| a).collect::<Vec<_>>(),
        vec![2, 3, 4, 5, 6, 7]
    );

    let decd_item = rb.dequeue();
    assert_eq!(rb[decd_item], 2);
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(
        rb.iter().map(|&a| a).collect::<Vec<_>>(),
        vec![3, 4, 5, 6, 7]
    );

    let popped_item = rb.pop_rear();
    assert_eq!(rb[popped_item], 7);
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![3, 4, 5, 6]);
}

#[test]
fn ring_buffer_enq_tests() {
    let mut rb = RingBuffer::from(RingSpecifier::MakeEmpty(vec![1, 2, 3, 4]));

    assert_eq!(rb.is_empty(), true);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![]);

    let idx = rb.enqueue();
    rb[idx] = -1;
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![-1]);

    let idx = rb.enqueue();
    rb[idx] = -2;
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![-1, -2]);

    let idx = rb.enqueue();
    rb[idx] = -3;
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![-1, -2, -3]);

    let idx = rb.enqueue();
    rb[idx] = -4;
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), true);
    assert_eq!(
        rb.iter().map(|&a| a).collect::<Vec<_>>(),
        vec![-1, -2, -3, -4]
    );

    let idx = rb.dequeue();
    assert_eq!(rb[idx], -1);
    assert_eq!(rb.is_empty(), false);
    assert_eq!(rb.is_full(), false);
    assert_eq!(rb.len(), 3);
    assert_eq!(rb.iter().map(|&a| a).collect::<Vec<_>>(), vec![-2, -3, -4]);

    let front = rb.front();
    let next_idx = rb.peek_next();
    assert_eq!(rb[front], -2);
    assert_eq!(rb[next_idx], -3);
}
