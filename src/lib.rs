// a max heap I guess?

pub struct Heap {
    values: Vec<i32>,
}

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

        // move last to top
        let last_value = self.values.pop().unwrap();
        self.values[0] = last_value;

        // bubble down top value
        let mut idx: usize = 0;
        let len = self.values.len();
        while idx < len {
            let left_child_idx = Heap::get_left_child_idx(idx);
            let right_child_idx = Heap::get_right_child_idx(idx);
            let mut largest_child_idx = idx;

            if left_child_idx < len && self.values[left_child_idx] > self.values[largest_child_idx]
            {
                largest_child_idx = left_child_idx;
            } else if right_child_idx < len
                && self.values[right_child_idx] > self.values[largest_child_idx]
            {
                largest_child_idx = right_child_idx;
            }

            if largest_child_idx == idx {
                break;
            }

            self.values.swap(idx, largest_child_idx);
            idx = largest_child_idx;
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
            let parent_idx = Heap::get_parent_idx(idx);
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
