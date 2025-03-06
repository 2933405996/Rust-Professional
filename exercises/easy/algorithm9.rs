/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::{self, Default};
use std::fmt::Display;

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
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn heapify_up(&mut self) {
        let mut idx = self.count - 1;
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
            } else {
                break;
            }
            idx = parent_idx;
        }
    }

    pub fn heapify_down(&mut self) {
        let mut idx = 0;
        while idx < self.count {
            let left_child_idx = self.left_child_idx(idx);
            let right_child_idx = self.right_child_idx(idx);
            let mut min_idx = idx;

            if left_child_idx < self.count
                && (self.comparator)(&self.items[left_child_idx], &self.items[min_idx])
            {
                min_idx = left_child_idx;
            }
            if right_child_idx < self.count
                && (self.comparator)(&self.items[right_child_idx], &self.items[min_idx])
            {
                min_idx = right_child_idx;
            }
            if min_idx == idx {
                break;
            }
            self.items.swap(idx, min_idx);
            idx = min_idx;
        }
    }

    pub fn add(&mut self, value: T) {
        //TODO

        self.items.push(value);
        self.count += 1;
        self.heapify_up();
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);

        if left_child_idx < self.count
            && (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx])
        {
            left_child_idx
        } else {
            right_child_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default + Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if !self.is_empty() {
            let value = self.items.remove(0);
            self.count -= 1;
            if !self.is_empty() {
                self.heapify_down();
            }
            return Some(value);
        }
        None
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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
