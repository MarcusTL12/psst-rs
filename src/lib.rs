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
            k: 10,
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

        if self.n_s == 0 {
            return None;
        }

        let mut lo = 0;
        let mut hi = self.n_s - 1;

        while hi - lo > 1 {
            let mid = (hi + lo) / 2;

            match self.data[mid].0.cmp(k) {
                Greater => hi = mid - 1,
                Equal => return Some(mid),
                Less => lo = mid + 1,
            }
        }

        if self.data[lo].0.eq(k) {
            Some(lo)
        } else if hi != lo && self.data[hi].0.eq(k) {
            Some(hi)
        } else {
            None
        }
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

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(i) = self.search(&k) {
            let (_, other_v) = self.data.get_mut(i).unwrap();

            Some(std::mem::replace(other_v, v))
        } else {
            self.data.push((k, v));

            if self.n_u() > self.k {
                println!("sorted at n = {}", self.len());
                self.data.sort_by(|(a, _), (b, _)| a.cmp(b));
                self.n_s = self.len();
                self.k = self.get_opt_num_unsorted();
                println!("set k to {}", self.k);
            }

            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Psst;

    #[test]
    fn it_works() {
        let mut s = Psst::new();

        s.insert("Heisann!".to_owned(), 76);
        s.insert("Hadesann!".to_owned(), 69);

        s.insert("Marcus".to_owned(), 1);
        s.insert("Takvam".to_owned(), 2);
        s.insert("Lexander".to_owned(), 3);

        s.insert("Ylva".to_owned(), 3);
        s.insert("Os".to_owned(), 28);

        s.insert("Sverre".to_owned(), 23);
        s.insert("Emanuel".to_owned(), 18);
        s.insert("Dårflot".to_owned(), 321);
        s.insert("Olsen".to_owned(), 17);

        s.insert("231456".to_owned(), 11235);
        s.insert("...".to_owned(), 987);

        s.insert("68".to_owned(), 25);
        s.insert("5".to_owned(), 655458);
        s.insert("6".to_owned(), 666);
        assert_eq!(s.insert("Marcus".to_owned(), 7), Some(1));
        s.insert("7".to_owned(), 1215);
        s.insert("8".to_owned(), 6654842);
        s.insert("9".to_owned(), 655458);
        s.insert("10".to_owned(), 666);
        s.insert("12".to_owned(), 666);

        println!("{:?}", s);

        assert_eq!(s.get(&"Marcus".to_owned()), Some(&7));
    }
}
