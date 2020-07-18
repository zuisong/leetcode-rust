/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/description/
 *
 * algorithms
 * Medium (38.77%)
 * Total Accepted:    7.1K
 * Total Submissions: 18.2K
 * Testcase Example:  '[1,2,3,3,4,4,5]'
 *
 * 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
 *
 * 示例 1:
 *
 * 输入: 1->2->3->3->4->4->5
 * 输出: 1->2->5
 *
 *
 * 示例 2:
 *
 * 输入: 1->1->1->2->3
 * 输出: 2->3
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
        Self::delete_duplicates_node(head, false)
    }
    pub fn delete_duplicates_node(
        head: Option<Box<ListNode>>,
        delete_current: bool,
    ) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                let n = node.next;
                if n.is_none() || n.as_ref().unwrap().val != node.val {
                    let new_next = Self::delete_duplicates_node(n, false);
                    if delete_current {
                        new_next
                    } else {
                        node.next = new_next;
                        Some(node)
                    }
                } else {
                    Self::delete_duplicates_node(n, true)
                }
            }
            _ => None,
        }
    }
}

fn main() {
    let mut l1 = ListNode::new(1);
    let mut l2 = ListNode::new(2);
    let mut l3 = ListNode::new(3);
    let mut l4 = ListNode::new(4);
    let mut l5 = ListNode::new(4);
    let l6 = ListNode::new(6);
    l5.next = Some(Box::new(l6));
    l4.next = Some(Box::new(l5));
    l3.next = Some(Box::new(l4));
    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));
    dbg!(&l1);
    let l1 = Solution::delete_duplicates(None);
    dbg!(&l1);
}

struct Solution {}
