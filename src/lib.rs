pub mod template;

// Use this file to add helper functions and additional modules.

pub struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> MinHeap {
        MinHeap { heap: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> MinHeap {
        MinHeap {
            heap: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.heap.push(value);
        self.heapify_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }

        let result = self.heap.swap_remove(0);
        self.heapify_down(0);
        Some(result)
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let par = (index - 1) / 2;
        if self.heap[par] > self.heap[index] {
            self.heap.swap(par, index);
            self.heapify_up(par);
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut min = index;

        if left < self.heap.len() && self.heap[left] < self.heap[min] {
            min = left;
        }

        if right < self.heap.len() && self.heap[right] < self.heap[min] {
            min = right;
        }

        if min != index {
            self.heap.swap(index, min);
            self.heapify_down(min);
        }
    }
}

impl Default for MinHeap {
    fn default() -> MinHeap {
        MinHeap::new()
    }
}
