use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum BinarySearchTree<T> {
    Empty,
    Node(Rc<Node<T>>),
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    item: T,
    left: BinarySearchTree<T>,
    right: BinarySearchTree<T>,
}

impl<T: Clone + PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }
    pub fn make_leaf(item: T) -> Self {
        BinarySearchTree::Node(Rc::new(Node {
            item,
            left: BinarySearchTree::Empty,
            right: BinarySearchTree::Empty,
        }))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            BinarySearchTree::Empty => true,
            _ => false,
        }
    }

    pub fn item(&self) -> Option<&T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(item) => Some(&item.item),
        }
    }

    pub fn left(&self) -> Option<&Self> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(item) => Some(&item.left),
        }
    }

    pub fn right(&self) -> Option<&Self> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(item) => Some(&item.right),
        }
    }

    pub fn find<K>(&self, key: &K) -> Option<&T>
    where
        T: PartialOrd<K>,
    {
        match self {
            BinarySearchTree::Node(node) => match node.item.partial_cmp(key) {
                Some(Ordering::Equal) => Some(&node.item),
                Some(Ordering::Greater) => node.left.find(key),
                Some(Ordering::Less) => node.right.find(key),
                None => None,
            },
            _ => None,
        }
    }

    pub fn insert(&self, item: T) -> Self {
        match self {
            BinarySearchTree::Empty => BinarySearchTree::make_leaf(item),
            BinarySearchTree::Node(node) => match item.partial_cmp(&node.item) {
                None => todo!(),
                Some(Ordering::Equal) => BinarySearchTree::Node(Rc::new(Node {
                    item,
                    left: node.left.clone(),
                    right: node.right.clone(),
                })),
                Some(Ordering::Less) => BinarySearchTree::Node(Rc::new(Node {
                    item: node.item.clone(),
                    left: node.left.insert(item),
                    right: node.right.clone(),
                })),
                Some(Ordering::Greater) => BinarySearchTree::Node(Rc::new(Node {
                    item: node.item.clone(),
                    left: node.left.clone(),
                    right: node.right.insert(item),
                })),
            },
        }
    }

    pub fn delete<K>(&self, key: &K) -> Option<Self>
    where
        T: PartialOrd<K>,
    {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(node) => match node.item.partial_cmp(key) {
                None => todo!(),
                Some(Ordering::Equal) if node.left.is_empty() => Some(node.right.clone()),
                Some(Ordering::Equal) if node.right.is_empty() => Some(node.left.clone()),
                Some(Ordering::Equal) => {
                    let x = self.right().unwrap().smallest().unwrap();
                    Some(BinarySearchTree::Node(Rc::new(Node {
                        item: x.clone(),
                        left: node.left.clone(),
                        right: node.right.delete(x)?,
                    })))
                }
                Some(Ordering::Less) => Some(BinarySearchTree::Node(Rc::new(Node {
                    item: node.item.clone(),
                    left: node.left.clone(),
                    right: node.right.delete(key)?,
                }))),
                Some(Ordering::Greater) => Some(BinarySearchTree::Node(Rc::new(Node {
                    item: node.item.clone(),
                    left: node.left.delete(key)?,
                    right: node.right.clone(),
                }))),
            },
        }
    }

    fn smallest(&self) -> Option<&T> {
        match self {
            BinarySearchTree::Empty => None,
            BinarySearchTree::Node(node) if node.left.is_empty() => Some(&node.item),
            BinarySearchTree::Node(node) => node.left.smallest(),
        }
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
    fn empty_tree() {
        let bst = BinarySearchTree::<()>::new();
        assert_eq!(bst.item(), None);
        assert!(bst.left().is_none());
        assert!(bst.right().is_none());
        assert_eq!(bst.find(&()), None);
    }

    #[test]
    fn items_not_in_the_tree_yield_none() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        assert_eq!(bst.find(&2), None)
    }

    #[test]
    fn first_inserted_item_stays_at_the_root() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        let bst = bst.insert(2);
        assert_eq!(bst.item(), Some(&1));
    }

    #[test]
    fn greater_item_becomes_right_child() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        let bst = bst.insert(2);
        assert_eq!(bst.right().unwrap().item(), Some(&2));
    }

    #[test]
    fn lesser_item_becomes_left_child() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(2);
        let bst = bst.insert(1);
        assert_eq!(bst.left().unwrap().item(), Some(&1));
    }

    #[test]
    fn equal_item_replaces_node() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(1);
        let bst = bst.insert(1);
        assert_eq!(bst.item(), Some(&1));
        assert!(bst.left().unwrap().item().is_none());
        assert!(bst.right().unwrap().item().is_none());
    }

    #[test]
    fn can_lookup_left_and_right_children() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(2);
        let bst = bst.insert(1);
        let bst = bst.insert(3);
        assert_eq!(bst.find(&2), Some(&2));
        assert_eq!(bst.find(&1), Some(&1));
        assert_eq!(bst.find(&3), Some(&3));
    }

    #[test]
    fn lookup_in_degenerate_tree() {
        let bst = BinarySearchTree::new();
        let bst = bst.insert(10);
        let bst = bst.insert(20);
        let bst = bst.insert(30);
        let bst = bst.insert(28);
        let bst = bst.insert(26);
        let bst = bst.insert(24);
        let bst = bst.insert(22);
        assert_eq!(bst.find(&22), Some(&22));
        assert_eq!(bst.find(&30), Some(&30));
        assert_eq!(bst.find(&20), Some(&20));
    }

    #[test]
    fn delete_empty_tree() {
        let bst = BinarySearchTree::<()>::new();
        assert_eq!(bst.clone().delete(&()), None);
    }

    #[test]
    fn delete_sole_root() {
        let empty = BinarySearchTree::new();
        let bst = empty.insert(42);
        assert_eq!(bst.delete(&42), Some(empty));
    }

    #[test]
    fn delete_bigger_leaf() {
        let empty = BinarySearchTree::new();
        let root = empty.insert(1);
        let bst = root.insert(2);
        assert_eq!(bst.delete(&2), Some(root));
    }

    #[test]
    fn delete_smaller_leaf() {
        let empty = BinarySearchTree::new();
        let root = empty.insert(2);
        let bst = root.insert(1);
        assert_eq!(bst.delete(&1), Some(root));
    }

    #[test]
    fn delete_root_with_right_child() {
        let empty = BinarySearchTree::new();
        let root = empty.insert(1);
        let bst = root.insert(2);
        assert_eq!(bst.delete(&1), Some(BinarySearchTree::new().insert(2)));
    }

    #[test]
    fn delete_root_with_left_child() {
        let empty = BinarySearchTree::new();
        let root = empty.insert(2);
        let bst = root.insert(1);
        assert_eq!(bst.delete(&2), Some(BinarySearchTree::new().insert(1)));
    }

    #[test]
    fn delete_root_with_both_children() {
        let empty = BinarySearchTree::new();
        let bst = empty.insert(2);
        let bst = bst.insert(1);
        let bst = bst.insert(3);
        assert_eq!(
            bst.delete(&2),
            Some(BinarySearchTree::new().insert(3).insert(1))
        );
    }

    #[test]
    fn delete_root_of_deep_tree() {
        let bst = BinarySearchTree::new()
            .insert(50)
            .insert(25)
            .insert(75)
            .insert(10)
            .insert(40)
            .insert(60)
            .insert(90);
        let actual = bst.delete(&50).unwrap();
        let expected = BinarySearchTree::new()
            .insert(60)
            .insert(25)
            .insert(75)
            .insert(10)
            .insert(40)
            .insert(90);
        assert_eq!(actual, expected);
    }
}
