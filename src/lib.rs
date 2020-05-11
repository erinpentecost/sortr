#[cfg(feature = "bogo")]
pub mod bogo;
#[cfg(feature = "bubble")]
pub mod bubble;
#[cfg(feature = "heap")]
pub mod heap;
#[cfg(feature = "insertion")]
pub mod insertion;
#[cfg(feature = "merge")]
pub mod merge;

use cfg_if::cfg_if;

pub mod signature {

    /// Sortable elements supertrait. This trait is implemented
    /// on all types that implement its own traits.
    pub trait Sortable: PartialOrd + Copy {}
    impl<T> Sortable for T where T: PartialOrd + Copy {}

    /// This defines the interface for comparison-based sorting algorithms.
    pub trait Sort<T: Sortable> {
        fn sort(&self, list: &[T]) -> Vec<T>;
    }
}

/// Returns a list of all concrete implementations of signature::sort
/// in this module.
pub fn concrete_sorts<T: signature::Sortable>() -> Vec<Box<dyn signature::Sort<T>>> {
    let mut sorts: Vec<Box<dyn signature::Sort<T>>> = vec![];
    cfg_if! {
        if #[cfg(feature = "bogo")] {
            sorts.push(Box::new(bogo::Bogo));
        }
    }
    cfg_if! {
        if #[cfg(feature = "bubble")] {
            sorts.push(Box::new(bubble::Bubble));
        }
    }
    cfg_if! {
        if #[cfg(feature = "heap")] {
            sorts.push(Box::new(heap::Heap));
        }
    }
    cfg_if! {
        if #[cfg(feature = "insertion")] {
            sorts.push(Box::new(insertion::Insertion));
        }
    }
    cfg_if! {
        if #[cfg(feature = "merge")] {
            sorts.push(Box::new(merge::Merge));
        }
    }
    sorts
}
