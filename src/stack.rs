const STACK_CAPACITY: usize = 64;

#[derive(Debug)]
pub struct Stack {
    pub data: Vec<StackItem>,
    size: u32,
}

#[derive(Debug)]
pub enum StackItem {
    I32(i32),
    String(u32),
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(STACK_CAPACITY),
            size: 0,
        }
    }

    pub fn push(&mut self, item: StackItem) {
        self.size += 1;
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.size -= 1;
        self.data.pop()
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}
