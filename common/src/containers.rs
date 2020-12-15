/// A drop-in replacement for simple uses of HashMap<usize, T>, that uses a Vec as
/// backing storage. In contrast to HashMap, the representation is not sparse. May
/// result in better performance if enough memory is available.
pub struct VecMap<T> {
    data: AutoVec<Option<T>>,
}

/// A Vec that automatically grows when an access would be out of bounds.
pub struct AutoVec<T> {
    data: Vec<T>,
}

impl<T> VecMap<T> {
    pub fn new() -> Self {
        Self {
            data: AutoVec::new(),
        }
    }

    pub fn pre_allocate(&mut self, max_key: usize) {
        self.data.pre_allocate(max_key)
    }

    pub fn insert(&mut self, key: usize, value: T) -> Option<T> {
        self.data.set(key, Some(value))
    }

    pub fn remove(&mut self, key: &usize) -> Option<T> {
        self.data.set(*key, None)
    }

    pub fn get(&mut self, key: &usize) -> Option<&T> {
        self.data.get(*key).as_ref()
    }
}

impl<T: Default> AutoVec<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn pre_allocate(&mut self, max_index: usize) {
        self.ensure_index(max_index)
    }

    pub fn set(&mut self, index: usize, value: T) -> T {
        self.ensure_index(index);
        std::mem::replace(&mut self.data[index], value)
    }

    pub fn get(&mut self, index: usize) -> &T {
        self.ensure_index(index);
        &self.data[index]
    }

    fn ensure_index(&mut self, index: usize) {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, T::default)
        }
    }
}
