use std::rc::Rc;
use std::cmp::{Ord, Ordering};

pub enum BinarySearchTree<T> {
    Empty,
    Node(Rc<(T, BinarySearchTree<T>, BinarySearchTree<T>)>),
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }

    pub fn item(&self) -> Option<&T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(item) => Some(&item.0),
        }
    }

    pub fn find<K>(&self, key: &K) -> Option<&T>
    where T: PartialOrd<K>
    {
        match self {
            BinarySearchTree::Node(node) => {
                match node.0.partial_cmp(key) {
                    Some(Ordering::Equal) => Some(&node.0),
                    Some(_) => {
                        node.1.find(key).or_else(||node.2.find(key))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn insert(&self, item: T) -> Self {
        BinarySearchTree::Node(Rc::new((item, self.clone(), BinarySearchTree::Empty)))
    }
}

impl<T> Clone for BinarySearchTree<T> {
    fn clone(&self) -> Self {
        match self {
            BinarySearchTree::Empty => BinarySearchTree::Empty,
            BinarySearchTree::Node(item) => BinarySearchTree::Node(item.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_key_in_empty_tree_returns_nothing() {
        let bst = BinarySearchTree::<()>::new();
        assert_eq!(bst.find(&()), None);
    }

    #[test]
    fn items_can_be_inserted_and_retrieved() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(42);
        assert_eq!(bst.find(&42), Some(&42))
    }

    #[test]
    fn items_not_in_the_tree_yield_none() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        assert_eq!(bst.find(&2), None)
    }

    #[test]
    fn can_retrieve_two_inserted_items() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        let bst = bst.insert(2);
        assert_eq!(bst.find(&1), Some(&1));
        assert_eq!(bst.find(&2), Some(&2));
        assert_eq!(bst.find(&3), None);
    }
}
