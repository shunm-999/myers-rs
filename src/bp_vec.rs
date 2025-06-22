struct BpVec<T> {
    inner: Vec<T>,
}

impl<T> BpVec<T> {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn get(&self, index: i64) -> &T {
        let len = self.inner.len();

        let index = {
            if index < 0 {
                (len as i64 + index) as usize
            } else {
                index as usize
            }
        };
        &self.inner[index]
    }

    fn set(&mut self, index: i64, value: T) {
        let len = self.inner.len();

        let index = {
            if index < 0 {
                (len as i64 + index) as usize
            } else {
                index as usize
            }
        };
        self.inner[index] = value;
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
    fn test_push_and_get() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        assert_eq!(bp_vec.len(), 3);
        assert_eq!(*bp_vec.get(0), 1);
        assert_eq!(*bp_vec.get(1), 2);
        assert_eq!(*bp_vec.get(2), 3);
    }

    #[test]
    fn test_negative_index() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        assert_eq!(*bp_vec.get(-1), 3);
        assert_eq!(*bp_vec.get(-2), 2);
        assert_eq!(*bp_vec.get(-3), 1);
    }

    #[test]
    fn test_set() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        bp_vec.set(1, 5);
        assert_eq!(*bp_vec.get(1), 5);

        bp_vec.set(-1, 10);
        assert_eq!(*bp_vec.get(-1), 10);
        assert_eq!(*bp_vec.get(2), 10);
    }

    #[test]
    fn test_clone() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.push(2);
        bp_vec.push(3);

        let cloned = bp_vec.clone();
        assert_eq!(cloned.len(), bp_vec.len());
        assert_eq!(*cloned.get(0), *bp_vec.get(0));
        assert_eq!(*cloned.get(1), *bp_vec.get(1));
        assert_eq!(*cloned.get(2), *bp_vec.get(2));
    }

    #[test]
    fn test_string_values() {
        let mut bp_vec = BpVec::new();
        bp_vec.push("hello".to_string());
        bp_vec.push("world".to_string());

        assert_eq!(bp_vec.get(0), "hello");
        assert_eq!(bp_vec.get(-1), "world");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_positive() {
        let bp_vec: BpVec<i32> = BpVec::new();
        bp_vec.get(0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds_negative() {
        let mut bp_vec = BpVec::new();
        bp_vec.push(1);
        bp_vec.get(-2);
    }
}
