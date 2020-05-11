use super::signature::*;

/// Merge without intermediate buffers.
pub struct Merge;

impl Merge {
    /// Merge two sublists that are adjacent and already sorted.
    /// The left sublist is left_start..mid
    /// The right sublist is mid..right_end
    fn merge<T: Sortable>(
        &self,
        list: &mut Vec<T>,
        left_start: usize,
        mid: usize,
        right_end: usize,
    ) {
        // elements in the left sublist to the left of 'left' head are sorted
        let mut left = left_start;
        // elements in the right sublist to the right of 'right' head are sorted
        let mut right = mid;

        while left < right && right < right_end {
            if list[left] > list[right] {
                // swap needed, since the right head is
                // smaller than the left head.
                // Insert the right head where the left head is,
                // shifting the left head and all other items left
                // of it the right.
                let right_head = list.remove(right);
                list.insert(left, right_head);
                right += 1;
            }
            left += 1;
        }
    }

    fn split<T: Sortable>(&self, list: &mut Vec<T>, left_start: usize, right_end: usize) {
        if left_start + 1 >= right_end {
            // just one element
            return;
        } else {
            let mid = (right_end + left_start) / 2;

            self.split(list, left_start, mid);

            self.split(list, mid, right_end);

            self.merge(list, left_start, mid, right_end);
        }
    }
}

impl<T: Sortable> Sort<T> for Merge {
    fn sort(&self, list: &[T]) -> Vec<T> {
        // Copy the immutable slice 'list' and store it
        // in a mutable vec that we will operate on.
        let mut sorted: Vec<T> = list.iter().cloned().collect();

        if sorted.len() <= 1 {
            return sorted;
        }

        // split into a bunch of sub lists and call merge on each.
        let len = sorted.len();
        self.split(&mut sorted, 0, len);

        return sorted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_one_in_order() {
        let m = Merge;

        let mut input = vec![0, 3];
        m.merge::<i32>(&mut input, 0, 1, 2);

        assert_eq!(input, vec![0, 3]);
    }

    #[test]
    fn merge_one_reverse_order() {
        let m = Merge;

        let mut input = vec![3, 0];
        m.merge::<i32>(&mut input, 0, 1, 2);

        assert_eq!(input, vec![0, 3]);
    }

    #[test]
    fn merge_missized_in_order() {
        let m = Merge;

        let mut input = vec![0, 3, 8];
        m.merge::<i32>(&mut input, 0, 1, 3);

        assert_eq!(input, vec![0, 3, 8]);
    }

    #[test]
    fn merge_reverse_order() {
        let m = Merge;

        let mut input = vec![30, 70, 90, 100, 21, 35, 83, 89];
        m.merge::<i32>(&mut input, 0, 4, 8);

        assert_eq!(input, vec![21, 30, 35, 70, 83, 89, 90, 100]);
    }
}
