impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut u=0;
        let mut n= u+1;
        if nums.is_empty() {
            return 0;
        }
        while(n<nums.len()){
            if(nums[u]==nums[n]){
                n = n+1;
            }
            else{
                u= u+1;
                nums[u]=nums[n];
                n= n+1;
            }
        }
        (u +1) as i32
        
    }
}