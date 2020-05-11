use super::signature::*;

/// Standard Floyd's.
pub struct Heap;

impl Heap {
    // left child of i => 2i
    // right child of i => 2i + 1
    // parent of i => floor((i-1)/2)
    /// Using Floyd's heap construction algorithm
    fn heapify<T: Sortable>(&self, list: &mut Vec<T>) {
        // don't need to sift_down leafs
        let smallest_parent: usize = list.len() / 2;
        let end = list.len() - 1;

        // Repair the heap by sifting down parent nodes
        for i in (0..smallest_parent + 1).rev() {
            self.sift_down(list, i, end);
        }
    }

    fn sift_down<T: Sortable>(&self, list: &mut Vec<T>, start: usize, end: usize) {
        let mut root = start;

        while 2 * root <= end {
            let left_child = 2 * root;
            let mut swap = root;
            // left child is bigger, so swap with root.
            if list[swap] < list[left_child] {
                swap = left_child;
            }
            // right child is bigger, so swap with root.
            if left_child + 1 <= end && list[swap] < list[left_child + 1] {
                swap = left_child + 1;
            }
            // root is biggest. since the children are valid
            // heaps, we can stop now.
            if swap == root {
                break;
            } else {
                // swap child and root, since child was bigger.
                list.swap(root, swap);
                // we don't know if root is valid,
                // so we might need to keep sifting it down.
                root = swap;
            }
        }
    }
}

impl<T: Sortable> Sort<T> for Heap {
    fn sort(&self, list: &[T]) -> Vec<T> {
        // Copy the immutable slice 'list' and store it
        // in a mutable vec that we will operate on.
        let mut sorted: Vec<T> = list.iter().cloned().collect();

        if sorted.len() <= 1 {
            return sorted;
        }

        self.heapify(&mut sorted);

        // Elements after i are sorted;
        // elements before i are organized as a heap.
        for i in (1..sorted.len()).rev() {
            // Move the biggest element (top of the heap, position 0)
            // to the end of the array. We are converting the heap
            // into a sorted list. The element we swap it with
            // won't be the biggest, so we broke the heap.
            sorted.swap(i, 0);
            // Fix the heap by sifting down the element in
            // the root position. Make sure we stay out of the
            // array partition we are using as the sorted list.
            self.sift_down(&mut sorted, 0, i - 1);
        }

        return sorted;
    }
}
