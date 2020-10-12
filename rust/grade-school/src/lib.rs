use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School<'a> {
    students: BTreeMap<u32, BTreeSet<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let entry = self.students.entry(grade).or_insert(BTreeSet::new());
        (*entry).insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().map(|&k| k).collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let grade = self.students.get(&grade);
        match grade {
            Some(set) => Some((*set).iter().map(|name| String::from(*name)).collect()),
            None => None,
        }
    }
}
