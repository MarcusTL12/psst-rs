#[derive(Debug, Clone)]
pub struct Psst<K: Ord, V> {
    n_s: usize, // Current size of sorted block
    k: usize,   // Max size of unsorted block
    data: Vec<(K, V)>,
}

impl<K: Ord, V> Psst<K, V> {
    pub fn new() -> Self {
        Self {
            n_s: 0,
            k: 0,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn n_u(&self) -> usize {
        self.len() - self.n_s
    }

    fn binary_search(&self, k: &K) -> Option<usize> {
        use std::cmp::Ordering::*;

        let mut lo = 0;
        let mut hi = self.n_s;

        while lo < hi {
            let mid = (hi + lo) / 2;

            match self.data[mid].0.cmp(k) {
                Less => hi = mid,
                Equal => return Some(mid),
                Greater => lo = mid,
            }
        }

        None
    }

    fn linear_search(&self, k: &K) -> Option<usize> {
        self.data
            .iter()
            .enumerate()
            .skip(self.n_s)
            .find(|(_, (x, _))| x.eq(k))
            .map(|(i, _)| i)
    }

    fn search(&self, k: &K) -> Option<usize> {
        self.binary_search(k).or_else(|| self.linear_search(k))
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        if let Some(i) = self.search(k) {
            self.data.get(i).map(|(_, x)| x)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        if let Some(i) = self.search(k) {
            self.data.get_mut(i).map(|(_, x)| x)
        } else {
            None
        }
    }

    fn get_opt_num_unsorted(&self) -> usize {
        let n = self.data.len() as f64;

        (n * n * n.log2()).powf(1.0 / 3.0) as usize
    }

    pub fn insert(&mut self, k: K, mut v: V) -> Option<V> {
        if let Some(i) = self.search(&k) {
            self.data.get_mut(i).map(|(_, x)| x).replace(&mut v);
            Some(v)
        } else {
            self.data.push((k, v));

            if self.n_u() > self.k {
                self.data.sort_by(|(a, _), (b, _)| a.cmp(b));
                self.n_s = self.len();
                self.k = self.get_opt_num_unsorted();
            }

            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
