use std::cmp::PartialEq;
use std::iter::IntoIterator;

pub trait RichVec<T> {
    fn push_if_not_exists(&mut self, v: T) -> bool;

    fn extend_if_not_exists<I: IntoIterator<Item = T>>(&mut self, i: I);
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
}


pub trait RichStringVec {
  fn extend_with_prefix<I, T>(&mut self, i: I, prefix: T)
  where I: IntoIterator<Item = String>, T: AsRef<str>;
}

impl RichStringVec for Vec<String> {
  fn extend_with_prefix<I, T>(&mut self, i: I, prefix: T)
    where I: IntoIterator<Item = String>, T: AsRef<str> {
        i.into_iter()
        .for_each(|item| {
          self.push_if_not_exists(format!("{}{}", prefix.as_ref(), item));
        })
    }
}