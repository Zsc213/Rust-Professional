/*
    heap
    This question requires you to implement a binary heap function
*/

use std::clone::Clone;
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO

        self.items.push(value);
        self.count += 1;
        if self.count > 1 {
            self.up_float(self.count);
        }
    }

    fn up_float(&mut self, idx: usize) {
        if idx <= 1 {
            return;
        }
        let p_idx = self.parent_idx(idx);
        if !(self.comparator)(&self.items[p_idx], &self.items[idx]) {
            // 不满足平衡条件
            self.swap(p_idx, idx);
            self.up_float(p_idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.left_child_idx(idx) > self.count {
            return idx;
        } else if self.right_child_idx(idx) > self.count {
            return self.left_child_idx(idx);
        } else {
            return self.count - 1;
        }
    }

    fn down_float(&mut self, idx: usize) {
        if idx >= self.count {
            return;
        }
        let l_idx = self.left_child_idx(idx);
        let r_idx = self.right_child_idx(idx);
        if l_idx > self.count {
            return;
        } else if r_idx > self.count {
            // left
            if !(self.comparator)(&self.items[idx], &self.items[l_idx]) {
                self.swap(idx, l_idx);
                self.down_float(l_idx);
            }
        } else if (self.comparator)(&self.items[l_idx], &self.items[r_idx])
            && !(self.comparator)(&self.items[idx], &self.items[l_idx])
        {
            // left
            self.swap(idx, l_idx);
            self.down_float(l_idx);
        } else if (self.comparator)(&self.items[r_idx], &self.items[l_idx])
            && !(self.comparator)(&self.items[idx], &self.items[r_idx])
        {
            // right
            self.swap(idx, r_idx);
            self.down_float(r_idx);
        }
    }

    fn swap(&mut self, idx_a: usize, idx_b: usize) {
        self.items.swap(idx_a, idx_b);
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            None
        } else if self.count == 1 {
            self.count -= 1;
            self.items.pop()
        } else {
            self.swap(1, self.count);
            self.count -= 1;

            let res = self.items.pop();
            self.down_float(1);
            res
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
