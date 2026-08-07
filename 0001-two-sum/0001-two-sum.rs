impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();

        for i in 0..n{
            for j in (i+1)..n{
                if numbers[i]+numbers[j]==target{
                    return vec![1+i as i32 , 1+j as i32];
                }
            }
        }
        vec![]
        
    }
}
