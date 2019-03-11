/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode-cn.com/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (57.73%)
 * Total Accepted:    14.7K
 * Total Submissions: 25.4K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
 *
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
 *
 *
 *
 * 示例:
 *
 * 给定 1->2->3->4, 你应该返回 2->1->4->3.
 *
 *
 */
use std::mem::{swap, replace};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        fn swap_p(mut node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if node.is_some() && node.as_ref().unwrap().next.is_some() {
                let pt1 = &mut node as *mut Option<Box<ListNode>>;
                let n1 = unsafe { &mut *pt1 };
                let n2 = &mut n1.as_mut().unwrap().next;

                let mut n3 = replace(&mut (*n2).as_mut().unwrap().next, node);

                let new_n3 = swap_p(n3);

                let mut n2 = replace(&mut *n2, None);

                (*n1).as_mut().unwrap().next = new_n3;

                n2
            } else {
                node
            }
        }
        let head = swap_p(head);
        head
    }
}


fn main() {
    let mut l1 = ListNode::new(1);
    let mut l2 = ListNode::new(2);
    let mut l3 = ListNode::new(3);
    let mut l4 = ListNode::new(4);
    let mut l5 = ListNode::new(5);
    let mut l6 = ListNode::new(6);
    l5.next = Some(Box::new(l6));
    l4.next = Some(Box::new(l5));
    l3.next = Some(Box::new(l4));
    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));
    dbg!(&l1);
    let n = Solution::swap_pairs(Some(Box::new(l1)));
    dbg!(n);
}


struct Solution {}