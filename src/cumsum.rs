use std::ops::{Add, Sub};

/// CumSum is a struct that contains vec of cumulative sum.
/// ```
/// let x = vec![1, 2, 3];
/// let cs = k0i::cumsum::CumSum::new(&x);
/// assert_eq!(cs.sec_ref(), &[0, 1, 3, 6]);
/// ```
pub struct CumSum<T> {
    sec: Vec<T>,
}

impl<T> CumSum<T>
where
    T: PartialOrd + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T]) -> Self {
        let mut sec: Vec<T> = vec![T::default(); v.len() + 1];
        for i in 0..v.len() {
            sec[i + 1] = sec[i] + v[i];
        }
        Self { sec }
    }
    // return reference own sec.
    pub fn sec_ref(&self) -> &Vec<T> {
        &self.sec
    }
    // clone and return own sec.
    pub fn sec(&self) -> Vec<T> {
        self.sec.clone()
    }
    /// return sum of the section that containes [left, right)
    /// ```
    /// let x = vec![1,2,3,4,5];
    /// let cs = k0i::cumsum::CumSum::new(&x);
    /// assert_eq!(cs.sum(2,4),x[2]+x[3]);
    /// assert_eq!(cs.sum(0,4),x.iter().take(4).sum());
    /// ```
    pub fn sum(&self, left: usize, right: usize) -> T {
        self.sec[right] - self.sec[left]
    }
}
