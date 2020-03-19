//给定一个整数数组 nums，求出数组从索引 i 到 j (i ≤ j) 范围内元素的总和，包含 i, j 两点。
//
// update(i, val) 函数可以通过将下标为 i 的数值更新为 val，从而对数列进行修改。
//
// 示例:
//
// Given nums = [1, 3, 5]
//
//sumRange(0, 2) -> 9
//update(1, 2)
//sumRange(0, 2) -> 8
//
//
// 说明:
//
//
// 数组仅可以在 update 函数下进行修改。
// 你可以假设 update 函数与 sumRange 函数的调用次数是均匀分布的。
//
// Related Topics 树状数组 线段树

//leetcode submit region begin(Prohibit modification and deletion)

struct NumArray {
    datas: Vec<i32>,
}

///
/// 解题思路，先够着一个完全二叉树，二叉树最底层，也就是叶节点存放原数据
/// 二叉树父节点等于两个子节点的和，依次向上填满二叉树
/// 空间上用长度是 2^(向上取整(log2(len))) * 2  
///
///
/// 改变一个子节点的数据的时候，依次向上更新父节点，直到根节点 时间复杂度O(lgN)
///
/// 范围内求和的时候，利用父节点数据求和, 时间复杂度可将为 O(lgN)
///
///

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut i = 0;
        while 2_i32.pow(i) < nums.len() as i32 {
            i += 1;
        }
        let len = 2_i32.pow(i) as usize;

        let mut datas = vec![0; len * 2];

        for (idx, val) in nums.into_iter().enumerate() {
            datas[len + idx] = val;
        }

        for i in (1..len).rev() {
            datas[i] = datas[2 * i] + datas[2 * i + 1];
        }

        let num_arr = NumArray { datas };
        return num_arr;
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        let mut index = self.datas.len() / 2 + i;
        self.datas[index] = val;

        while index != 1 {
            index = index / 2;
            self.datas[index] = self.datas[2 * index] + self.datas[2 * index + 1];
        }
    }
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        ///
        /// 完全二叉树当前层的长度
        /// datas 完全二叉树
        ///
        fn sum_range(datas: &Vec<i32>, idx: usize, len: usize) -> i32 {
            if idx == 0 {
                return datas[len];
            }
            return if idx % 2 == 1 {
                sum_range(datas, idx / 2, len / 2)
            } else {
                sum_range(datas, idx - 1, len) + datas[len + idx]
            };
        }

        let i = i as usize;
        let j = j as usize;
        let mut len = self.datas.len() / 2;
        let mut ii = len + i;

        return sum_range(&self.datas, j, len) - sum_range(&self.datas, i, len) + self.datas[ii];
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */

fn main() {
    let mut nums = NumArray::new(vec![1, 2, 4, 5, 99, 8, 3]);
    print_tree(&nums.datas);

    nums.update(1, 1000);
    print_tree(&nums.datas);

    nums.update(3, 1000);
    print_tree(&nums.datas);
    println!("{:?}", nums.sum_range(0, 1));
    println!("{:?}", nums.sum_range(0, 2));
    println!("{:?}", nums.sum_range(1, 3));
    println!("{:?}", nums.sum_range(1, 4));
}

fn print_tree(nums: &Vec<i32>) {
    let mut l = 1;

    for i in 1..nums.len() {
        if i as i32 == 2_i32.pow(l) - 1 {
            println!("{}", nums[i]);
            l += 1;
        } else {
            print!("{} ", nums[i])
        }
    }

    println!();
    println!();
}
