use super::signature::*;

extern crate rand;

use rand::Rng;

pub struct Bogo;

impl Bogo {
    /// Fisher-Yates shuffle.
    fn shuffle<T: Sortable>(&self, list: &mut Vec<T>) {
        if list.len() > 1 {
            let mut rng = rand::thread_rng();
            for i in 0..(list.len() - 1) {
                let j = rng.gen_range(i, list.len());
                list.swap(i, j);
            }
        }
    }

    /// Checks if the list is sorted.
    fn sorted<T: Sortable>(&self, list: &mut Vec<T>) -> bool {
        if list.len() <= 1 {
            return true;
        }

        let mut j = 0;
        for i in 1..list.len() {
            if list[j] > list[i] {
                return false;
            }
            j = i;
        }

        return true;
    }
}

impl<T: Sortable> Sort<T> for Bogo {
    fn sort(&self, list: &[T]) -> Vec<T> {
        // Copy the immutable slice 'list' and store it
        // in a mutable vec that we will operate on.
        let mut sorted: Vec<T> = list.iter().cloned().collect();

        while !self.sorted(&mut sorted) {
            self.shuffle(&mut sorted);
        }

        sorted
    }
}
