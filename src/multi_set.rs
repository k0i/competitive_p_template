use std::collections::BTreeSet;

const INF: usize = 1 << 60;
#[derive(Debug)]
pub struct MultiSet<T>
where
    T: Ord,
{
    set: BTreeSet<(T, usize)>,
    index: usize,
}

impl<T> MultiSet<T>
where
    T: Ord + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            index: 0,
        }
    }

    pub fn lower_bound(&self, x: T) -> Option<usize> {
        let cnt = self.set.range(..(x, INF)).count();
        if cnt == self.set.len() {
            None
        } else {
            Some(cnt)
        }
    }

    pub fn lower_kth(&self, x: T, k: usize) -> Option<&(T, usize)> {
        self.set.range(..=(x, INF)).rev().nth(k)
    }

    pub fn higer_kth(&self, x: T, k: usize) -> Option<&(T, usize)> {
        self.set.range((x, 0)..).nth(k)
    }

    pub fn upper_bound(&self, x: T) -> Option<usize> {
        let cnt = self.set.range(..=(x, INF)).count();
        if cnt == self.set.len() {
            None
        } else {
            Some(cnt)
        }
    }

    pub fn insert(&mut self, x: T) {
        self.set.insert((x, self.index));
        self.index += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}
