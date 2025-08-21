use std::cmp;

impl Solution {
    // 从(0,0)出发，只能走对角线到达(n,n)，因此对角线上的全都是第一个孩子收集
    // (n-1,0)出发的孩子只能收集到对角线左下角的水果，(0,n-1)出发的孩子只能收集到对角线右上角的水果
    // f[i][j] 代表从(n-1,0)出发，走到(i,j)收集到的最大水果数量
    // f[i][j]=max(f[i-1][j-1],f[i+1][j-1],f[i][j-1]) + fruits[i][j]
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut f = vec![vec![0; n]; n];

        // first calculate the maximum number of fruits the children (n-1,0) collects
        f[n - 1][0] = fruits[n - 1][0];
        for j in 1..n {
            for i in cmp::max(n - j - 1,j)..n {
                f[i][j] = cmp::max(f[i - 1][j - 1], f[i][j - 1]);
                if i + 1 < n {
                    f[i][j] = cmp::max(f[i][j], f[i + 1][j - 1]);
                }
                if i != j {
                    f[i][j] += fruits[i][j];
                }
            }
        }

        let mut collected_fruits = f[n - 1][n - 1];

        for i in 0..n {
            f[i].fill(0);
        }

        f[0][n - 1] = fruits[0][n - 1];
        for i in 1..n {
            for j in cmp::max(n - i - 1, i)..n {
                f[i][j] = cmp::max(f[i - 1][j], f[i - 1][j - 1]);
                if j + 1 < n {
                    f[i][j] = cmp::max(f[i][j], f[i - 1][j + 1]);
                }
                if i != j {
                    f[i][j] += fruits[i][j];
                }
            }
        }

        collected_fruits += f[n - 1][n - 1];

        for i in 0..n {
            collected_fruits += fruits[i][i];
        }

        return collected_fruits;
    }
}
