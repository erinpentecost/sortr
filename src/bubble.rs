use super::signature::*;
pub struct Bubble;

impl<T: Sortable> Sort<T> for Bubble {
    fn sort(&self, list: &[T]) -> Vec<T> {
        // Copy the immutable slice 'list' and store it
        // in a mutable vec that we will operate on.
        let mut sorted: Vec<T> = list.iter().cloned().collect();

        if sorted.len() <= 1 {
            return sorted;
        }

        // At the end of each iteration,
        // elements after end are sorted.
        for end in (1..sorted.len()).rev() {
            for i in 0..end {
                let j = i + 1;
                if sorted[i] > sorted[j] {
                    sorted.swap(i, j);
                }
            }
        }

        return sorted;
    }
}
