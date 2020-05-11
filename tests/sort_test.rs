macro_rules! sort_tests {
    ($mname:ident: $value:expr, $($name:ident: $sorter:path,)*) => {
    mod $mname {
        use sortr::*;
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let concrete_sort = Box::new($sorter) as Box<dyn signature::Sort<i32>>;

                // assert_eq only works on arrays of max length 32.
                let sorted = &concrete_sort.sort(input);
                let matches = expected.iter().zip(sorted.iter());
                for (exp, act) in matches {
                    assert_eq!(exp, act);
                }
            }
        )*
    }
    }
}

sort_tests! {
    empty: (&[] as &[i32], &[] as &[i32]),
    bogo: bogo::Bogo,
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}

sort_tests! {
    one: (&[1], &[1]),
    bogo: bogo::Bogo,
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}

sort_tests! {
    in_order: (&[1, 2, 3, 6, 8], &[1, 2, 3, 6, 8]),
    bogo: bogo::Bogo,
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}

sort_tests! {
    reverse_order: (&[8, 6, 3, 2, 1], &[1, 2, 3, 6, 8]),
    bogo: bogo::Bogo,
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}

sort_tests! {
    mixed_order: (&[8, 3, 6, 2, 1], &[1, 2, 3, 6, 8]),
    bogo: bogo::Bogo,
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}

sort_tests! {
    large_set: (&[-4631, 2413, -4363, -299, -769, -1441, 4280, -377, 2772, -2418, 3320, 4188, 1799, 1270, -3282, 2649, 818, -1277, -3802, -1789, -2654, 3887, 428, -4264, -1374, -2319, -3622, 3120, 4432, 2092, -2929, -2651, -342, -4408, 1241, 3992, -3450, -1639, 1779, 2777, -3031, 357, 895, -4532, 3590, -497, 1867, -3146, 3710, -3221, -2697, 3308, -2447, -2053, -1628, 2360, -2022, 1594, -4610, -1903, -3928, -3548, -2254, -3323, 1717, -1896, -888, -491, 1836, -2221, 3771, 4241, 2499, -2440, -2764, -4589, -1982, 3441, 3930, -2763, 4201, 123, -3673, 2251, -2834, 432, -1284, 434, 1320, 2436, 3972, 461, -3355, -4890, -3560, 565, -4255, -3941, -963, 4333], &[-4890, -4631, -4610, -4589, -4532, -4408, -4363, -4264, -4255, -3941, -3928, -3802, -3673, -3622, -3560, -3548, -3450, -3355, -3323, -3282, -3221, -3146, -3031, -2929, -2834, -2764, -2763, -2697, -2654, -2651, -2447, -2440, -2418, -2319, -2254, -2221, -2053, -2022, -1982, -1903, -1896, -1789, -1639, -1628, -1441, -1374, -1284, -1277, -963, -888, -769, -497, -491, -377, -342, -299, 123, 357, 428, 432, 434, 461, 565, 818, 895, 1241, 1270, 1320, 1594, 1717, 1779, 1799, 1836, 1867, 2092, 2251, 2360, 2413, 2436, 2499, 2649, 2772, 2777, 3120, 3308, 3320, 3441, 3590, 3710, 3771, 3887, 3930, 3972, 3992, 4188, 4201, 4241, 4280, 4333, 4432]),
    // bogo: bogo::Bogo, // bogo is too slow.
    insertion: insertion::Insertion,
    bubble: bubble::Bubble,
    merge: merge::Merge,
    heap: heap::Heap,
}
