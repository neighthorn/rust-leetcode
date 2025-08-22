impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        // 找到最小的i，最大的i，最小的j，最大的j
        let n = grid.len();
        let m = grid[0].len();
        let mut minimum_i_index = n;
        let mut maximum_i_index = n;
        let mut minimum_j_index = m;
        let mut maximum_j_index = m;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    minimum_i_index = i;
                    break;
                }
            }
            if minimum_i_index != n {
                break;
            }
        }

        for i in (0..n).rev() {
            for j in 0..m {
                if grid[i][j] == 1 {
                    maximum_i_index = i;
                    break;
                }
            }
            if maximum_i_index != n {
                break;
            }
        }

        for j in 0..m {
            for i in 0..n {
                if grid[i][j] == 1 {
                    minimum_j_index = j;
                    break;
                }
            }
            if minimum_j_index != m {
                break;
            }
        }

        for j in (0..m).rev() {
            for i in 0..n {
                if grid[i][j] == 1 {
                    maximum_j_index = j;
                    break;
                }
            }
            if maximum_j_index != m {
                break;
            }
        }

        return ((maximum_j_index - minimum_j_index + 1) * (maximum_i_index - minimum_i_index + 1)) as i32;
    }
}
