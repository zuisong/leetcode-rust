/*

给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。

k 是一个正整数，它的值小于或等于链表的长度。

如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

示例 :

给定这个链表：1->2->3->4->5

当 k = 2 时，应当返回: 2->1->4->3->5

当 k = 3 时，应当返回: 3->2->1->4->5

说明 :

你的算法只能使用常数的额外空间。
你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reverse-nodes-in-k-group
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }
        let next_head = {
            let mut temp = &mut head;
            for _ in 0..k {
                if temp.is_none() {
                    return head;
                }
                temp = &mut temp.as_mut().unwrap().next;
            }
            Self::reverse_k_group(temp.take(), k)
        };

        let mut node = head;
        let next = node.as_mut().unwrap().next.take();
        node.as_mut().unwrap().next = next_head;
        let mut new_head = node;
        node = next;
        for _i in 1..k {
            let next = node.as_mut().unwrap().next.take();
            node.as_mut().unwrap().next = new_head;
            new_head = node;
            node = next;
        }
        new_head
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
    let new_head = Solution::reverse_k_group(head, 6);
    dbg!(new_head);
}

struct Solution {}
