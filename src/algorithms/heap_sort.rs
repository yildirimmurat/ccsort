use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub fn sort(arr: &mut [String]) {
    let mut heap = BinaryHeap::new();

    // wrapped in Reverse to sort lexicographically
    for item in arr.iter() {
        heap.push(Reverse(item.clone()));
    }

    let mut i = 0;
    while let Some(Reverse(item)) = heap.pop() {
        arr[i] = item;
        i += 1;
    }
}

