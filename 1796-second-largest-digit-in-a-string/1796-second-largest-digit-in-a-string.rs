impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut second_largest:i32= -1;
        let mut largest :i32=-1;
        for c in s.chars(){
            if let Some(digit) = c.to_digit(10){
                let d = digit as i32;
                if d>largest{
                    second_largest = largest;
                    largest = d;
                }else if d< largest && d>second_largest{
                    second_largest = d;
                }
            }
        }
        second_largest
    }
}