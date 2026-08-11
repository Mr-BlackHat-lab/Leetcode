impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
// Case 1: Empty input
        if nums.is_empty() {
            return nums;
        }
        
        let mut pos: Vec<i32> = Vec::new();
        let mut neg: Vec<i32> = Vec::new();

        // Separate and square at the same time
        for i in nums {
            if i < 0 {
                neg.push(i * i);
            } else {
                pos.push(i * i);
            }
        }
        
        // Case 2: Only positive numbers
        if neg.is_empty() {
            return pos;
        }
        
        // Case 3: Only negative numbers
        if pos.is_empty() {
            neg.reverse();
            return neg;
        }
        
        // Case 4: Mix of positive and negative
        
        // Use `isize` so `l` can safely hit -1 without crashing Rust
        let mut l: isize = (neg.len() as isize) - 1; 
        let mut r: usize = 0; 
        
        let mut final_arr: Vec<i32> = Vec::new();
        
        while l >= 0 && r < pos.len() {
            // Cast `l` back to `usize` just for indexing
            if neg[l as usize] <= pos[r] {
                final_arr.push(neg[l as usize]);
                l -= 1;
            } else {
                final_arr.push(pos[r]);
                r += 1;
            }            
        }
        
        // Catch any remaining negative numbers
        while l >= 0 {
            final_arr.push(neg[l as usize]);
            l -= 1;
        }
        
        // Catch any remaining positive numbers
        while r < pos.len() {
            final_arr.push(pos[r]);
            r += 1;
        }
        
        final_arr
    }
}