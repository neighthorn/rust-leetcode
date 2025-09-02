impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut sorted_points = points.clone();
        // 按照横坐标从大到小排序
        sorted_points.sort_by(|a,b| {
            b[0].cmp(&a[0]).then_with(|| a[1].cmp(&b[1]))
        });

        let mut count: i32 = 0;
        for i in 0..n-1 {
            // 左边的点的高度不能低于当前点
            let mut prev_y = 51;
            let mut prev_x = sorted_points[i][0];

            if sorted_points[i+1][0] == sorted_points[i][0] {
                count += 1;
                // println!("({},{}), found above point", sorted_points[i][0], sorted_points[i][1]);
                prev_y = sorted_points[i+1][1];
            }

            for j in i+1..n {
                if sorted_points[j][0] == prev_x {
                    continue;
                }
                if sorted_points[j][1] == sorted_points[i][1] {
                    // 如果存在和points[i][1]相同的点
                    // println!("({},{}), found left point", sorted_points[i][0], sorted_points[i][1]);
                    count += 1;
                    break;
                }
                if sorted_points[j][1] < sorted_points[i][1] {
                    continue;
                }
                if sorted_points[j][1] < prev_y {
                    count += 1;
                    // println!("({},{}), found left above point", sorted_points[i][0], sorted_points[i][1]);
                    prev_y = sorted_points[j][1];
                }
                prev_x = sorted_points[j][0];
            }
        }

        count
    }
}
