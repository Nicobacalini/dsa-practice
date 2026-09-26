impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut pila = Vec::new();
        for c in s.chars(){
            if c == '(' || c == '[' || c == '{'{
                pila.push(c);
            } else if c == ']' || c == ')' || c == '}'{
                match pila.pop() {
                    Some('(') if c == ')' => {}
                    Some ('[') if c == ']' => {}
                    Some ('{') if c == '}' => {}
                    _ => return false,
                }
            }
        }
        pila.is_empty()
    }
}