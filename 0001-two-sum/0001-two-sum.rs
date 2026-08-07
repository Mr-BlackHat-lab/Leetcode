impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        if numbers.is_empty() {
            return Vec::new();
        }

        loop {
            let sum = numbers[l] + numbers[r];

            if sum == target {
                // return 1 based so
                return vec![(l + 1) as i32, (r + 1) as i32];
            }

            if l >= r {
                return Vec::new();
            }

            if sum > target {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
}
