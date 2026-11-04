use crate::PsstMap;

pub struct PsstSet<T: Ord>(PsstMap<T, ()>);

impl<T: Ord> PsstSet<T> {
    pub fn new() -> Self {
        Self(PsstMap::new())
    }

    pub fn insert(&mut self, x: T) -> bool {
        self.0.insert(x, ()).is_none()
    }

    pub fn contains(&self, x: &T) -> bool {
        self.0.get(x).is_some()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord> Default for PsstSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for PsstSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut s = Self::new();

        for x in iter {
            s.insert(x);
        }

        s
    }
}
