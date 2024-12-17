pub fn sort(arr: &mut [String]) {
    let len = arr.len();
    if len < 2 { return; } // If the array has fewer than two elements, no need to sort
    quick_sort(arr, 0, len - 1);
}

fn quick_sort(arr: &mut [String], start: usize, end: usize) {
    if start < end {
        // Partition the array and get the pivot index
        let pivot = partition(arr, start, end);

        // Recursively apply the sort to the left and right subarrays
        if pivot > start { // Ensure that we are not going out of bounds
            quick_sort(arr, start, pivot - 1); // Left side
        }
        if pivot < end { // Ensure that we are not going out of bounds
            quick_sort(arr, pivot + 1, end); // Right side
        }
    }
}

fn partition(arr: &mut [String], start: usize, end: usize) -> usize {
    let pivot = arr[end].clone(); // The pivot element is the last element

    let mut i = start; // Initialize i at the start of the array

    for j in start..end {
        if arr[j] < pivot {
            arr.swap(i, j); // Swap elements if arr[j] is less than the pivot
            i += 1;
        }
    }

    // Finally, swap the pivot element into its correct position
    arr.swap(i, end);
    i // Return the index of the pivot
}
