use std::cmp::PartialEq;
use std::iter::IntoIterator;

pub trait RichVec<T> {
    fn push_if_not_exists(&mut self, v: T) -> bool;

    fn extend_if_not_exists<I: IntoIterator<Item = T>>(&mut self, i: I);

    fn dedup_ordered(self) -> Vec<T>;
}

impl<T: PartialEq> RichVec<T> for Vec<T> {
    fn push_if_not_exists(&mut self, v: T) -> bool {
        if !self.contains(&v) {
            self.push(v);
            true
        } else {
            false
        }
    }

    fn extend_if_not_exists<I: IntoIterator<Item = T>>(&mut self, i: I) {
        i.into_iter().for_each(|item| {
            self.push_if_not_exists(item);
        })
    }
    
    fn dedup_ordered(self) -> Vec<T> {
        let mut copy: Vec<T> = Vec::with_capacity(self.len());
        copy.extend_if_not_exists(self);
        copy
    }
}
