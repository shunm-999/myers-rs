use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub(crate) struct BpVec<T> {
    inner: Vec<T>,
}

impl<T: Default + Clone> BpVec<T> {
    pub(crate) fn new() -> Self {
        Self { inner: vec![] }
    }
    pub(crate) fn new_with_capacity(capacity: usize, default: T) -> Self {
        Self {
            inner: vec![default; capacity],
        }
    }

    fn get_index(&self, index: i64) -> usize {
        let len = self.inner.len();

        if index < 0 {
            (len as i64 + index) as usize
        } else {
            index as usize
        }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Default + Clone> Index<i64> for BpVec<T> {
    type Output = T;
    fn index(&self, index: i64) -> &Self::Output {
        let index = self.get_index(index);
        &self.inner[index]
    }
}

impl<T: Default + Clone> IndexMut<i64> for BpVec<T> {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let index = self.get_index(index);
        &mut self.inner[index]
    }
}

impl<'a, T: 'static> Iterator for &'a BpVec<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.iter().next()
    }
}

impl<T: Clone> Clone for BpVec<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bp_vec: BpVec<i32> = BpVec::new();
        assert!(bp_vec.is_empty());
        assert_eq!(bp_vec.len(), 0);
    }

    #[test]
    fn test_push_and_index() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        assert_eq!(bp_vec.len(), 3);
        assert_eq!(bp_vec[0], 1);
        assert_eq!(bp_vec[1], 2);
        assert_eq!(bp_vec[2], 3);
    }

    #[test]
    fn test_negative_index() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        assert_eq!(bp_vec[-1], 3);
        assert_eq!(bp_vec[-2], 2);
        assert_eq!(bp_vec[-3], 1);
    }

    #[test]
    fn test_index_mut() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        bp_vec[1] = 5;
        assert_eq!(bp_vec[1], 5);

        bp_vec[-1] = 10;
        assert_eq!(bp_vec[-1], 10);
        assert_eq!(bp_vec[2], 10);
    }

    #[test]
    fn test_clone() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        let cloned = bp_vec.clone();
        assert_eq!(cloned.len(), bp_vec.len());
        assert_eq!(cloned[0], bp_vec[0]);
        assert_eq!(cloned[1], bp_vec[1]);
        assert_eq!(cloned[2], bp_vec[2]);
    }

    #[test]
    fn test_string_values() {
        let mut bp_vec = BpVec::new();
        bp_vec.push("hello".to_string());
        bp_vec.push("world".to_string());

        assert_eq!(bp_vec[0], "hello");
        assert_eq!(bp_vec[-1], "world");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_positive() {
        let bp_vec: BpVec<i32> = BpVec::new();
        let _ = bp_vec[0];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_negative() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        let _ = bp_vec[-2];
    }

    #[test]
    fn test_mixed_positive_negative_index() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(10);
        bp_vec.push(20);
        bp_vec.push(30);

        // 正のインデックスと負のインデックスが同じ要素を指すことを確認
        assert_eq!(bp_vec[0], bp_vec[-3]);
        assert_eq!(bp_vec[1], bp_vec[-2]);
        assert_eq!(bp_vec[2], bp_vec[-1]);
    }

    #[test]
    fn test_index_mut_with_negative_index() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(100);
        bp_vec.push(200);

        // 負のインデックスで値を変更
        bp_vec[-1] = 999;
        assert_eq!(bp_vec[-1], 999);
        assert_eq!(bp_vec[1], 999);

        // 正のインデックスで値を変更
        bp_vec[0] = 888;
        assert_eq!(bp_vec[0], 888);
        assert_eq!(bp_vec[-2], 888);
    }
}
