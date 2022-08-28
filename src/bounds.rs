use std::cmp::Ordering;

// Returns an iterator to specified bound that pointing to the first element in the range.
pub trait Bound<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
    fn b_search(&self, x: T) -> usize;
}

impl<T: Ord> Bound<T> for [T] {
    /// Returns an iterator pointing to the first element in the range [first,last) which does not compare less than val.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(4), 2);
    /// ```
    /// If the candidate is larger than set's larget item,returns set's last index.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(1000), 3);
    /// ```
    fn lower_bound(&self, x: T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        // Returns last item's index if x is larger than self's largest item.
        if low == self.len() {
            low - 1
        } else {
            low
        }
    }
    /// Returns an iterator pointing to the first element in the range [first,last) which compares greater than val.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(4), 3);
    /// ```
    /// If the candidate is larger than set's larget item,returns set's last index.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![2, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(1000), 3);
    /// ```
    fn upper_bound(&self, x: T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Greater => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
        // Returns last item's index if x is larger than self's smallest item.
        if low == self.len() {
            low - 1
        } else {
            low
        }
    }
    /// Returns an smallest index i such that x <= self[i] .
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    /// assert_eq!(vec.b_search(51), 3);
    /// assert_eq!(vec.b_search(1), 0);
    /// assert_eq!(vec.b_search(0), 0);
    /// assert_eq!(vec.b_search(910), vec.len() - 1);
    /// assert_eq!(vec.b_search(1000), vec.len());
    /// ```
    fn b_search(&self, x: T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] >= x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
