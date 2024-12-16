pub fn sort(arr: &mut [String]) {
    // Find the maximum string length
    let max_len = arr.iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    // Perform counting sort for each digit position, starting from the least significant (rightmost)
    for digit_pos in (0..max_len).rev() {
        counting_sort(arr, digit_pos);
    }
}

fn counting_sort(arr: &mut [String], digit_pos: usize) {
    // Create 256 buckets for ASCII characters
    let mut buckets = vec![Vec::new(); 256]; // 256 for ASCII characters

    // Go through each string and place it in the correct bucket based on the character at digit_pos
    for s in arr.iter() {
        if digit_pos < s.len() {
            // Safely get the character at the digit position
            if let Some(char_at_digit_pos) = s.chars().nth(digit_pos) {
                // Convert the character to its Unicode value
                let idx = char_at_digit_pos as usize;

                // Ensure the index stays within the bounds of the buckets (0..255 for ASCII)
                if idx < 256 {
                    buckets[idx].push(s.clone());
                }
            }
        } else {
            // If the string is too short for the current digit position, we will place it in the first bucket
            buckets[0].push(s.clone());  // Shorter strings are placed in the first bucket (i.e., 'smaller' strings)
        }
    }

    // Rebuild the array from the buckets
    let mut idx = 0;
    for bucket in buckets.iter() {
        for val in bucket {
            arr[idx] = val.clone();
            idx += 1;
        }
    }
}
