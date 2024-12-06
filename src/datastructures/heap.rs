#[derive(Debug)]
pub struct Heap {
    pub values: Vec<i32>,
}

// implementation of a max heap
impl Heap {
    pub fn new() -> Heap {
        return Heap { values: Vec::new() };
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.values.is_empty() {
            return None;
        }

        let top = self.values.first().unwrap().clone();
        if self.values.len() == 1 {
            self.values.clear();
            return Some(top);
        }

        // move last to top & bubble down top value
        self.values[0] = self.values.pop().unwrap().clone();
        let mut idx: usize = 0;
        let len = self.values.len();
        while idx < len {
            let left_child_idx: usize = Heap::get_left_child_idx(idx);
            let right_child_idx = Heap::get_right_child_idx(idx);
            let mut largest_child_idx = idx;

            if left_child_idx < len
                && self.values.get(left_child_idx).unwrap()
                    > self.values.get(largest_child_idx).unwrap()
            {
                largest_child_idx = left_child_idx;
            }

            if right_child_idx < len
                && self.values.get(right_child_idx).unwrap()
                    > self.values.get(largest_child_idx).unwrap()
            {
                largest_child_idx = right_child_idx;
            }

            if largest_child_idx == idx {
                break;
            } else {
                self.values.swap(idx, largest_child_idx);
                idx = largest_child_idx;
            }
        }

        return Some(top);
    }

    pub fn push(&mut self, value: i32) {
        self.values.push(value);

        let mut idx = self.values.len() - 1;
        if idx == 0 {
            return;
        }

        while idx > 0 {
            let parent_idx: usize = Heap::get_parent_idx(idx);
            if value
                > self
                    .values
                    .get(parent_idx)
                    .expect("expected parent value")
                    .clone()
            {
                self.values.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn get_parent_idx(idx: usize) -> usize {
        return (idx - 1) / 2;
    }

    fn get_left_child_idx(idx: usize) -> usize {
        return (idx * 2) + 1;
    }

    fn get_right_child_idx(idx: usize) -> usize {
        return Heap::get_left_child_idx(idx) + 1;
    }
}
