use std::cmp;

impl Solution {
    const direction_op : [[i32; 2]; 4] = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    const expected_num : [i32; 2] = [0, 2];
    // direction代表现在朝哪个方向行走，turn_time代表还剩多少次转弯机会
    pub fn dfs(direction: usize, turn_time: i32, curr_i: i32, curr_j : i32, curr_len: usize, grid: &Vec<Vec<i32>>) -> i32 {
        let nxt_i = curr_i + Self::direction_op[direction][0];
        let nxt_j = curr_j + Self::direction_op[direction][1];

        if nxt_i >= grid.len() as i32 || nxt_j >= grid[0].len() as i32 || nxt_i < 0 || nxt_j < 0 {
            return curr_len as i32;
        }
        if grid[nxt_i as usize][nxt_j as usize] != Self::expected_num[curr_len & 1] {
            return curr_len as i32;
        }

        let mut res = Self::dfs(direction, turn_time, nxt_i, nxt_j, curr_len + 1, &grid);
        if turn_time > 0 {
            // res = cmp::max(res, Self::dfs((direction + 1) % 4, turn_time - 1, nxt_i, nxt_j, curr_len + 1, &grid));
            res = cmp::max(res, Self::dfs((direction + 4 - 1) % 4, turn_time - 1, nxt_i, nxt_j, curr_len + 1, &grid));
        }

        return res;
    }

    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut result = 0;

        for i in 0..n {
            for j in 0..m {
                // 枚举起点
                if grid[i][j] != 1 { continue; }
                // 只有1可以作为起点
                for direction in 0..4 {
                    result = cmp::max(result, Self::dfs(direction, 1, i as i32, j as i32, 1, &grid));
                    result = cmp::max(result, Self::dfs(direction, 0, i as i32, j as i32, 1, &grid));
                }
            }
        }

        return result;
    }
}
