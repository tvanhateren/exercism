use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct CustomSet<T: Hash + Eq + Copy> {
    entries: HashMap<T, ()>,
}

impl<T: Hash + Eq + Copy> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }

    fn ne(&self, other: &Self) -> bool {
        self.entries != other.entries
    }
}

impl<T: Hash + Eq + Copy> CustomSet<T> {
    fn new_from_vec(entries: Vec<T>) -> Self {
        let mut set = CustomSet {
            entries: HashMap::new(),
        };

        for entry in entries {
            set.add(entry);
        }

        set
    }

    pub fn new(entries: &[T]) -> Self {
        let mut set = CustomSet {
            entries: HashMap::new(),
        };

        for &entry in entries {
            set.add(entry);
        }

        set
    }

    pub fn add(&mut self, value: T) {
        self.entries.entry(value).or_insert(());
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.entries.contains_key(item)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.entries.keys().all(|k| other.contains(k))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.entries.keys().any(|k| other.contains(k))
    }
}

impl<T: Hash + Eq + Copy + Clone> CustomSet<T> {
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new_from_vec(
            self.entries
                .keys()
                .filter(|k| other.contains(k))
                .cloned()
                .collect(),
        )
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new_from_vec(
            self.entries
                .keys()
                .filter(|k| !other.contains(k))
                .cloned()
                .collect(),
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new_from_vec(
            self.entries
                .keys()
                .chain(other.entries.keys())
                .cloned()
                .collect(),
        )
    }
}
