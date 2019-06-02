/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU缓存机制
 */
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    m: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    key: i32,
    val: i32,
}

impl Node {
    fn new(key: i32, val: i32) -> Node {
        Node {
            prev: None,
            next: None,
            key,
            val,
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            m: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.m.get(&key) {
            Some(v) => {
                {
                    let prev = &v.borrow().prev;
                    let next = &v.borrow().next;
                    if next.is_none() {
                        // get the tail node,  do nothing
                        return RefCell::borrow(v).val;
                    }

                    let next = next.as_ref().unwrap().clone();

                    if prev.is_none() {
                        // get the head node
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    } else {
                        // get the middle node
                        prev.as_ref().unwrap().clone().borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(prev.as_ref().unwrap().clone());
                    }
                }
                let tail = self.tail.as_ref().unwrap().clone();
                v.clone().borrow_mut().prev = Some(tail.clone());
                v.clone().borrow_mut().next = None;
                tail.borrow_mut().next = Some(v.clone());
                self.tail = Some(v.clone());

                RefCell::borrow(v).val
            }
            None => -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let contains_key = self.m.contains_key(&key);
        if self.m.len() >= self.capacity && !contains_key {
            if self.capacity == 1 {
                self.m.clear();
                self.head = None;
                self.tail = None;
            } else {
                let head = self.head.clone().unwrap();
                let new_head = head.borrow().next.clone();
                self.head = new_head.clone();
                new_head.clone().unwrap().borrow_mut().prev = None;
                self.m.remove(&head.borrow().key);
            }
        }


        let node = Rc::new(RefCell::new(Node::new(key, value)));
        if contains_key {
            let node = self.m.get_mut(&key).unwrap();
            node.as_ref().borrow_mut().val = value;
            self.get(key);
        } else {
            self.m.insert(key, node.clone());
            match self.tail.clone() {
                None => {
                    self.head = Some(node.clone());
                    self.tail = Some(node.clone());
                }
                // 新添加的数据放到链表的末尾
                Some(tail) => {
                    node.clone().borrow_mut().prev = Some(tail.clone());
                    node.clone().borrow_mut().next = None;
                    tail.clone().borrow_mut().next = Some(node.clone());
                    self.tail = Some(node.clone());
                }
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[test]
fn test2() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.get(2), -1);
    cache.put(2, 6);
    assert_eq!(cache.get(1), -1);
    cache.put(1, 5);
    cache.put(1, 2);
    assert_eq!(cache.get(1), 2);
    assert_eq!(cache.get(2), 6);

//    ["LRUCache","get","put","get","put","put","get","get"]
//    [[2],[2],[2,6],[1],[1,5],[1,2],[1],[2]]
    //[null,-1,null,-1,null,null,2,6]
}

#[test]
fn test1() {

//    ["LRUCache","put","put","put","put","get","get"]
//    [[2],[2,1],[1,1],[2,3],[4,1],[1],[2]]
//

    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(1, 1);
    cache.put(2, 3);
    cache.put(4, 1);
    assert_eq!(cache.get(1), -1);
    assert_eq!(3, cache.get(2));
}

