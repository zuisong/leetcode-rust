/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第N个节点
 *
 * https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (32.30%)
 * Total Accepted:    30.8K
 * Total Submissions: 95.2K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
 *
 * 示例：
 *
 * 给定一个链表: 1->2->3->4->5, 和 n = 2.
 *
 * 当删除了倒数第二个节点后，链表变为 1->2->3->5.
 *
 *
 * 说明：
 *
 * 给定的 n 保证是有效的。
 *
 * 进阶：
 *
 * 你能尝试使用一趟扫描实现吗？
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ptr = &mut head as *mut Option<Box<ListNode>>;

        let mut pt1 = &head;
        // 第一个指针先往前走N步
        for _ in 0..n {
            pt1 = &pt1.as_ref().unwrap().next;
        }
        // 第二个指针开始走, 两个都每次走一步,
        // 第一个指针走到最底下的时候, 第二个指针所在的节点就是要删掉的节点
        while pt1.is_some() {
            pt1 = &pt1.as_ref().unwrap().next;
            if let Some(n) = unsafe { &mut (*ptr) } {
                ptr = &mut n.next as *mut Option<Box<ListNode>>
            }
        }

        unsafe { core::ptr::swap(ptr, &mut (*ptr).take().unwrap().next) }

        head
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
    dbg!(&head);
    let new_head = Solution::remove_nth_from_end(head, 2);
    dbg!(new_head);
}

struct Solution {}
