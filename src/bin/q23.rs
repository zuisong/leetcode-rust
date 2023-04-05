//Definition for singly-linked list.
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

struct Solution {}
// 合并 k 个排序链表，返回合并后的排序链表。请分析和描述算法的复杂度。
//
//示例:
//
//输入:
//[
//  1->4->5,
//  1->3->4,
//  2->6
//]
//输出: 1->1->2->3->4->4->5->6

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut r: Vec<i32> = lists
            .iter()
            .map(|it| {
                fn list_node_to_vec(node: &Option<Box<ListNode>>) -> Vec<i32> {
                    match node {
                        Some(node) => {
                            let mut v = list_node_to_vec(&node.next);
                            v.push(node.val);
                            v
                        }
                        None => Vec::new(),
                    }
                }
                list_node_to_vec(it)
            })
            .flat_map(Vec::into_iter)
            .collect();

        fn vec_to_list_node(mut v: Vec<i32>) -> Option<Box<ListNode>> {
            let mut res: Option<Box<ListNode>> = Option::None;
            while !v.is_empty() {
                let mut temp = ListNode::new(v.pop().unwrap());
                temp.next = res;
                res = Option::Some(Box::new(temp));
            }
            res
        }

        r.sort();
        vec_to_list_node(r)
    }
}

fn main() {
    let a = Some(Box::new(ListNode::new(21)));

    let b = Some(Box::new(ListNode::new(1)));
    let c = Some(Box::new(ListNode::new(1)));

    let res = Solution::merge_k_lists(vec![a, b, c]);

    dbg!(res);
}
