#[derive(Debug, Clone)]
pub struct Heap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap { data: vec![] }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        let mut heap = Heap { data };
        for i in 1..heap.data.len() {
            heap.trickle_up(i)
        }
        heap
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    pub fn push(&mut self, item: T) {
        let new_node = self.data.len();
        self.data.push(item);
        self.trickle_up(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let value = self.data.swap_remove(0);
            self.trickle_down(0);
            Some(value)
        }
    }

    fn trickle_up(&mut self, node: usize) {
        if node == 0 {
            return;
        }
        let p = self.parent(node);
        if self.data[p] < self.data[node] {
            self.data.swap(p, node);
            self.trickle_up(p);
        }
    }

    fn trickle_down(&mut self, node: usize) {
        let (l, r) = self.children(node);

        if l >= self.data.len() {
            return;
        }

        let child;

        if r >= self.data.len() {
            child = l;
        } else if self.data[l] > self.data[r] {
            child = l;
        } else {
            child = r;
        }

        if self.data[node] < self.data[child] {
            self.data.swap(child, node);
            self.trickle_down(child);
        }
    }

    fn parent(&self, node: usize) -> usize {
        (node - 1) / 2
    }

    fn children(&self, node: usize) -> (usize, usize) {
        (node * 2 + 1, node * 2 + 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_and_vec() {
        let heap = Heap::<()>::from_vec(vec![]);
        let vec = heap.into_vec();
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn largest_item_always_front() {
        let mut heap = Heap::new();
        heap.push(1);
        heap.push(3);
        heap.push(2);
        assert_eq!(heap.into_vec()[0], 3);
    }

    #[test]
    fn enforce_heap_invariant_on_vector_conversion() {
        let heap = Heap::from_vec(vec![1, 3, 2, 7, 9, 5]);
        assert_eq!(heap.into_vec(), vec![9, 7, 5, 1, 3, 2]);
    }

    #[test]
    fn pop_from_empty_heap() {
        let mut heap = Heap::<()>::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn pop_largest_value_from_heap() {
        let mut heap = Heap::from_vec(vec![1, 3, 5, 4, 2]);
        assert_eq!(heap.pop(), Some(5));
    }

    #[test]
    fn heap_property_restored_after_pop() {
        let mut heap = Heap::from_vec(vec![1, 3, 5, 4, 2, 6, 7, 8]);
        println!("{:?}", heap);
        heap.pop();
        assert_eq!(heap.into_vec(), vec![7, 4, 6, 1, 2, 3, 5])
    }
}
