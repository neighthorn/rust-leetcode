use std::cmp;

// A better solution: DP, dp[i+1][j+1]=min(dp[i+1][j], dp[i][j+1], dp[i][j])+1, 
// dp[i][j] represents the maximum side for the square with matrix[i][j] as the right-bottom corner
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut square_num: i32 = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prefix_sum = vec![vec![0; n]; m];
        
        // first calculate the prefix sum of matrix
        for i in 0..m {
            for j in 0..n {
                let mut sum = matrix[i][j];
                if i > 0 {
                    sum += prefix_sum[i - 1][j];
                }
                if j > 0 {
                    sum += prefix_sum[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sum -= prefix_sum[i - 1][j - 1];
                }
                prefix_sum[i][j] = sum;
            }
        }
        
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    continue;
                }
                // take the current element as the top-left corner of the square
                square_num += 1;
                let max_square_side = cmp::min(m - i, n - j);
                for square_side in 2..=max_square_side {
                    let mut square_sum = prefix_sum[i + square_side - 1][j + square_side - 1];
                    if i > 0 {
                        square_sum -= prefix_sum[i - 1][j + square_side - 1];
                    }
                    if j > 0 {
                        square_sum -= prefix_sum[i + square_side - 1][ j - 1];
                    }
                    if i > 0 && j > 0 {
                        square_sum += prefix_sum[i - 1][j - 1];
                    }
                    if square_sum == (square_side as i32) * (square_side as i32) {
                        square_num += 1;
                    }
                    else {
                        break;
                    }
                }
            }
        }

        return square_num;
    }
}