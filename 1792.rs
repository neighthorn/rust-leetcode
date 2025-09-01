use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct HeapElement(i32, i32);

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 增加一个学生的收益 (b-a)/b/(b+1)
        let current = (self.1 - self.0) as f64 / (self.1 as f64) / ((self.1 + 1) as f64);
        let other_val = (other.1 - other.0) as f64 / (other.1 as f64) / ((other.1 + 1) as f64);

        current.partial_cmp(&other_val).unwrap()
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HeapElement {}

impl PartialEq for HeapElement {
    fn eq(&self, other: &Self) -> bool {
        let current = (self.1 - self.0) as f64 / (self.1 as f64) / ((self.1 + 1) as f64);
        let other_val = (other.1 - other.0) as f64 / (other.1 as f64) / ((other.1 + 1) as f64);
        current == other_val
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();

        for class in &classes {
            heap.push(HeapElement(class[0], class[1]));
            // println!("{}/{}",class[0], class[1]);
        }

        for i in 0..extra_students {
            if let Some(mut top) = heap.pop() {
                top.0 += 1;
                top.1 += 1;
                // println!("{}/{}", top.0, top.1);
                heap.push(top);
            }
        }

        let mut sum_pass_ratio: f64 = 0.0;
        while let Some(element) = heap.pop() {
            // println!("{}/{}", element.0, element.1);
            sum_pass_ratio += element.0 as f64 / (element.1 as f64);
        }

        sum_pass_ratio /= classes.len() as f64;

        sum_pass_ratio
    }
}
