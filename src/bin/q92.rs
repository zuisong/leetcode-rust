/*
 * @lc app=leetcode.cn id=92 lang=rust
 *
 * [92] 反转链表 II
 *
 * https://leetcode-cn.com/problems/reverse-linked-list-ii/description/
 *
 * algorithms
 * Medium (41.93%)
 * Total Accepted:    7K
 * Total Submissions: 16.7K
 * Testcase Example:  '[1,2,3,4,5]\n2\n4'
 *
 * 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
 *
 * 说明:
 * 1 ≤ m ≤ n ≤ 链表长度。
 *
 * 示例:
 *
 * 输入: 1->2->3->4->5->NULL, m = 2, n = 4
 * 输出: 1->4->3->2->5->NULL
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
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        m: i32,
        n: i32,
    ) -> Option<Box<ListNode>> {
        fn ddd(
            mut node: Option<Box<ListNode>>,
            head: Option<Box<ListNode>>,
            mut tail: *mut Option<Box<ListNode>>,
            m: i32,
            n: i32,
            p: i32,
        ) -> Option<Box<ListNode>> {
            if p >= m && p <= n {
                let n1 = std::mem::replace(&mut node.as_mut().unwrap().next, None);
                if head.is_none() {
                    tail = &mut node;
                }
                node.as_mut().unwrap().next = head;

                return ddd(n1, node, tail, m, n, p + 1);
            } else {
                unsafe {
                    (*tail).as_mut().unwrap().next = node;
                }
                head
            }
        }

        if m < 1 {
            return None;
        }
        if m > n {
            return None;
        }
        if m == 1 {
            return ddd(head, None, &mut None, m, n, m);
        } else {
            let mut mid = head.as_mut();

            for _ in 1..(m - 1) {
                mid = mid.unwrap().next.as_mut();
            }
            let mut node_m: Option<Box<ListNode>>;

            node_m = std::mem::replace(&mut mid.as_mut().unwrap().next, None);
            node_m = ddd(node_m, None, &mut None, m, n, m);

            mid.as_mut().unwrap().next = node_m;
            return head;
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
    let new_head = Solution::reverse_between(head, 5, 6);
    dbg!(new_head);
}

struct Solution {}
