const INIT_STACK_CAPACITY: usize = 64;

#[derive(Debug)]
pub struct Stack {
    data: Vec<StackItem>,
}

#[derive(Debug)]
pub enum StackItem {
    I32(i32),
    String(u32),
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(INIT_STACK_CAPACITY),
        }
    }

    pub fn push(&mut self, item: StackItem) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}
