/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (44.31%)
 * Total Accepted:    20.5K
 * Total Submissions: 46K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
 *
 * 示例 1:
 *
 * 输入: 1->1->2
 * 输出: 1->2
 *
 *
 * 示例 2:
 *
 * 输入: 1->1->2->3->3
 * 输出: 1->2->3
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::delete_duplicates_node(head, false, true)
    }
    pub fn delete_duplicates_node(
        node: Option<Box<ListNode>>,
        delete_current: bool,
        is_first: bool,
    ) -> Option<Box<ListNode>> {
        match node {
            Some(mut node) => {
                let n = node.next;
                if n.is_none() || n.as_ref().unwrap().val != node.val {
                    let new_next = Self::delete_duplicates_node(n, false, true);
                    if delete_current && !is_first {
                        new_next
                    } else {
                        node.next = new_next;
                        Some(node)
                    }
                } else if is_first {
                    node.next = Self::delete_duplicates_node(n, true, false);
                    Some(node)
                } else {
                    Self::delete_duplicates_node(n, true, false)
                }
            }
            _ => None,
        }
    }
}

fn main() {
    let mut l1 = ListNode::new(1);
    let mut l2 = ListNode::new(1);
    let mut l3 = ListNode::new(2);
    let mut l4 = ListNode::new(4);
    let mut l5 = ListNode::new(4);
    let l6 = ListNode::new(6);
    l5.next = Some(Box::new(l6));
    l4.next = Some(Box::new(l5));
    l3.next = Some(Box::new(l4));
    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));
    //    dbg!(&l1);
    let l1 = Solution::delete_duplicates(Some(Box::new(l1)));
    dbg!(&l1);
}

struct Solution {}
