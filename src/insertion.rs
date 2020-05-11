use super::signature::*;
pub struct Insertion;

impl<T: Sortable> Sort<T> for Insertion {
    fn sort(&self, list: &[T]) -> Vec<T> {
        // Copy the immutable slice 'list' and store it
        // in a mutable vec that we will operate on.
        let mut sorted: Vec<T> = list.iter().cloned().collect();

        for i in 0..sorted.len() {
            // At the start of the loop iteration,
            // items <i are sorted.
            // Items at index <=i are sorted at the end
            // of each loop iteration.
            for j in 0..i {
                // Iterate on all sorted items
                // in order to find the index we need
                // to insert.
                if sorted[i] < sorted[j] {
                    // The new item is smaller than [j],
                    // which means [j] needs to be bumped up.
                    let popped_i = sorted.remove(i);
                    sorted.insert(j, popped_i);
                }
            }
        }

        sorted
    }
}
