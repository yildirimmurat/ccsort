pub mod radix_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod heap_sort;
pub mod random_sort;

pub enum Algorithm {
    RadixSort,
    MergeSort,
    QuickSort,
    HeapSort,
    RandomSort,
}

pub trait SortingAlgorithm {
    fn sort(&self, arr: &mut [String]); // Instance method (using `&self` to refer to the instance)
}

impl SortingAlgorithm for Algorithm {
    fn sort(&self, arr: &mut [String]) {
        match self {
            Algorithm::RadixSort => radix_sort::sort(arr),
            Algorithm::MergeSort => merge_sort::sort(arr),
            Algorithm::QuickSort => quick_sort::sort(arr),
            Algorithm::HeapSort => heap_sort::sort(arr),
            Algorithm::RandomSort => random_sort::sort(arr),
        }
    }
}
