use std::cmp;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut res = Vec::new();

        let diagonal_num = n + m - 1;

        for index_sum in 0..diagonal_num {
            let range = 0..=index_sum;
            if index_sum & 1 == 1 {
                for i in range {
                    let j = index_sum - i;
                    if i < m && j < n {
                        res.push(mat[i][j]);
                    }
                }
            } else {
                for i in range.rev() {
                    let j = index_sum - i;
                    if i < m && j < n {
                        res.push(mat[i][j]);
                    }
                }
            }
        }
        return res;
    }
}
