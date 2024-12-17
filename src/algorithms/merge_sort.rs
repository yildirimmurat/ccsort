pub fn sort(arr: &mut [String]) {
    let len = arr.len();
    if len > 1 {
        merge_sort(arr, 0, len - 1);
    }
}

pub fn merge_sort(arr: &mut [String], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;

        merge_sort(arr, left, mid); // Sort left half
        merge_sort(arr, mid + 1, right); // Sort right half

        merge(arr, left, mid, right); // Merge the sorted halves
    }
}

fn merge(arr: &mut [String], left: usize, mid: usize, right: usize) {
    let n1 = mid - left + 1; // Length of left subarray
    let n2 = right - mid;    // Length of right subarray

    // Temporary vectors for the left and right subarrays
    let mut L: Vec<String> = Vec::with_capacity(n1);
    let mut R: Vec<String> = Vec::with_capacity(n2);

    // Copy data to temp arrays L[] and R[]
    for i in 0..n1 {
        L.push(arr[left + i].clone());
    }
    for j in 0..n2 {
        R.push(arr[mid + 1 + j].clone());
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    // Merge the temp arrays back into the original array
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i].clone();
            i += 1;
        } else {
            arr[k] = R[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy any remaining elements of L[] (if any)
    while i < n1 {
        arr[k] = L[i].clone();
        i += 1;
        k += 1;
    }

    // Copy any remaining elements of R[] (if any)
    while j < n2 {
        arr[k] = R[j].clone();
        j += 1;
        k += 1;
    }
}
