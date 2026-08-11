impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    // Case 1: Empty input
    let n = nums.len();
        if n == 0 {
            return nums;
        }

        let mut result = vec![0; n]; // Create exactly one array of size N
        let mut l = 0;
        let mut r = n - 1;
        
        // Loop backwards from the end of the array to the beginning
        for p in (0..n).rev() {
            let left_val = nums[l].abs();
            let right_val = nums[r].abs();
            

            if left_val > right_val {
                result[p] = left_val * left_val;
                l += 1;
            } else {
                result[p] = right_val * right_val;
                if r > 0 {
                    r -= 1;
                }
            }
        }
        
        result
    }
}