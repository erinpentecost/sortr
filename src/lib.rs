pub mod bogo;
pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;

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
    vec![
        Box::new(bogo::Bogo),
        Box::new(bubble::Bubble),
        Box::new(heap::Heap),
        Box::new(insertion::Insertion),
        Box::new(merge::Merge),
    ]
}
