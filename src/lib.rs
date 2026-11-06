use crate::Entry::{Occupied, Vacant};

pub mod set;

#[derive(Debug, Clone)]
pub struct PsstMap<K: Ord, V> {
    n_s: usize, // Current size of sorted block
    k: usize,   // Max size of unsorted block
    data: Vec<(K, V)>,
}

pub enum Entry<'a, K: Ord, V> {
    Vacant(&'a mut PsstMap<K, V>, K),
    Occupied(&'a mut V),
}

impl<K: Ord, V> PsstMap<K, V> {
    pub fn new() -> Self {
        Self {
            n_s: 0,
            k: 16,
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn n_u(&self) -> usize {
        self.len() - self.n_s
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
        self.data[0..self.n_s]
            .binary_search_by_key(&k, |(k2, _)| k2)
            .ok()
            .or_else(|| self.linear_search(k))
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.search(k)
            .and_then(|i| self.data.get(i).map(|(_, x)| x))
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.search(k)
            .and_then(|i| self.data.get_mut(i).map(|(_, x)| x))
    }

    fn get_opt_num_unsorted(&self) -> usize {
        let n = self.data.len() as f64;

        (2.0 * n * n.log2()).sqrt().ceil() as usize
    }

    pub fn sort(&mut self) {
        self.data.sort_by(|(a, _), (b, _)| a.cmp(b));
        self.n_s = self.len();
        self.k = self.get_opt_num_unsorted();
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(i) = self.search(&k) {
            let (_, other_v) = self.data.get_mut(i).unwrap();

            Some(std::mem::replace(other_v, v))
        } else {
            if self.n_u() > self.k {
                self.sort();
            }

            // Make sure last inserted element is at the end of the collection
            // otherwise .entry() does not work
            self.data.push((k, v));

            None
        }
    }

    pub fn entry<'a>(&'a mut self, k: K) -> Entry<'a, K, V> {
        match self.get_mut(&k) {
            Some(v) => Occupied(v),
            None => Vacant(self, k),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.data.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.iter().map(|(_, v)| v)
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.n_u() == 0 {
            self.n_s = self.n_s.saturating_sub(1);
        }

        self.data.pop()
    }
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    pub fn or_insert(self, v: V) -> &'a mut V {
        match self {
            Occupied(v) => v,
            Vacant(s, k) => {
                s.insert(k, v);
                &mut s.data.last_mut().unwrap().1
            }
        }
    }
}

impl<K: Ord, V> Default for PsstMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> IntoIterator for PsstMap<K, V> {
    type Item = (K, V);

    type IntoIter =
        <std::vec::Vec<(K, V)> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for PsstMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut s = Self::new();

        for (k, v) in iter {
            s.insert(k, v);
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::PsstMap;

    #[test]
    fn it_works() {
        let mut s = PsstMap::new();

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
