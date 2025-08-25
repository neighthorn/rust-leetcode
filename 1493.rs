use std::cmp;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        // 三种最长子序列
        // 1. 原始序列全都是1
        // 2. 原始序列不全是1，原始序列中连续最长的1
        // 3. 原始序列中两个最长的1连续序列，中间有一个0

        let n = nums.len();
        let mut max_single_len = 0;
        let mut first_left = n;
        let mut first_len = 0;
        let mut second_left = n;
        let mut second_len = 0;

        for i in 0..n {
            if first_left == n {
                if nums[i] == 1 {
                    first_left = i;
                }
            }
            else if nums[i] == 0 {
                if first_len == 0 { // 在找first subarray
                    first_len = i - first_left;
                    max_single_len = cmp::max(max_single_len, first_len);
                    if i + 1 < n && nums[i + 1] == 1 { // 连续找到了第二个subarray
                        second_left = i + 1;
                        second_len = 0;
                    }
                    else { // 没有第二个subarray，那么重复找第一个subarray
                        first_left = n;
                        first_len = 0;
                    }
                }
                else if second_left != n && second_len == 0 { // 在找second array
                    second_len = i - second_left;
                    max_single_len = cmp::max(max_single_len, second_len + first_len);
                    // 更新first_array
                    if i + 1 < n && nums[i + 1] == 1{
                        first_left = second_left;
                        first_len = second_len;
                        second_left = i + 1;
                        second_len = 0;
                    }
                    else {
                        first_left = n;
                        first_len = 0;
                        second_left = n;
                        second_len = 0;
                    }
                }
            }
        }

        if second_left != n {
            max_single_len = cmp::max(max_single_len, n - second_left + first_len);
        }
        else if first_left != n {
            max_single_len = cmp::max(max_single_len, n - first_left);
        }

        return cmp::min(max_single_len as i32, (n - 1) as i32);
    }
}
