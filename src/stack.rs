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
            data: Vec::with_capacity(64),
            size: 0,
        }
    }

    pub fn push(&mut self, item: StackItem) {
        self.data.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<StackItem> {
        self.size -= 1;
        self.data.pop()
    }
}
