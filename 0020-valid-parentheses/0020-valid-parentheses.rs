impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' | '}' | ']' => {
                    if !Self::value_pop(ch, &mut stack) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }

    fn value_pop(ch: char, stack: &mut Vec<char>) -> bool {
        match stack.last() {
            Some(&top) if top == Self::convert_closing_to_opening(ch) => {
                stack.pop();
                true
            }
            _ => false,
        }
    }

    fn convert_closing_to_opening(ch: char) -> char {
        match ch {
            ')' => '(',
            '}' => '{',
            ']' => '[',
            _ => ' ',
        }
    }
}
