pub fn binary_search_interval(arr: &[usize], elt: usize) -> (usize, usize) {
    debug_assert!(elt >= arr[0]);
    // Invariant: elt in [arr.left; arr.right[
    let mut left = 0;
    let mut right = arr.len();
    while left + 1 < right {
        let mid = (left + right) / 2;
        let mid_elt = arr[mid];
        if elt < mid_elt {
            right = mid
        } else {
            left = mid
        }
    }
    (left, elt - arr[left])
}
