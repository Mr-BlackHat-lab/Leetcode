impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let min:i32 = i32::MIN/10;
        let max:i32 = i32::MAX/10;

        let mut num:i32 = x;
        let mut rev:i32 = 0;

        while num!=0{
            let last_digit = num % 10;
            if rev > max || (rev == max && last_digit > 7) {
                return 0;
            }

            if rev < min || (rev == min && last_digit < -8) {
                return 0;
            }
            rev = rev*10 + last_digit;
            num /=10; 
        }
   
        rev
    }
}