pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, val: i32) {
        todo!()
    }

    pub fn pop(&mut self) {
        todo!()
    }

    pub fn top(&self) -> i32 {
        todo!()
    }

    pub fn get_min(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }

    #[test]
    fn single_element() {
        let mut ms = MinStack::new();
        ms.push(5);
        assert_eq!(ms.get_min(), 5);
        assert_eq!(ms.top(), 5);
    }
}
