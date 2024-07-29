/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 *
 * https://leetcode-cn.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (58.20%)
 * Total Accepted:    36.6K
 * Total Submissions: 62.8K
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * 反转一个单链表。
 *
 * 示例:
 *
 * 输入: 1->2->3->4->5->NULL
 * 输出: 5->4->3->2->1->NULL
 *
 * 进阶:
 * 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
 *
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            node: Option<Box<ListNode>>,
            tail: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match node {
                Some(mut node) => {
                    let next = std::mem::replace(&mut node.next, tail);
                    reverse(next, Some(node))
                }
                None => tail,
            }
        }
        reverse(head, None)
    }

    pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                if node.next.is_none() {
                    Some(node)
                } else {
                    let next = node.next.take();
                    let mut new_head = Self::reverse_list2(next);

                    let mut temp = &mut new_head;

                    while temp.as_ref().unwrap().next.is_some() {
                        temp = &mut temp.as_mut().unwrap().next;
                    }
                    temp.as_mut().unwrap().next = Some(node);

                    new_head
                }
            }
            None => None,
        }
    }
}

fn main() {
    let mut l1 = ListNode::new(1);
    let mut l2 = ListNode::new(2);
    let mut l3 = ListNode::new(3);
    let mut l4 = ListNode::new(4);
    let mut l5 = ListNode::new(5);
    let l6 = ListNode::new(6);

    l5.next = Some(Box::new(l6));
    l4.next = Some(Box::new(l5));
    l3.next = Some(Box::new(l4));
    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));
    let head = Some(Box::new(l1));
    let new_head = Solution::reverse_list(head);
    dbg!(new_head);
}

struct Solution {}
