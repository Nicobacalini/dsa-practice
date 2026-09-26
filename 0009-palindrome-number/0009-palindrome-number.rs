impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let original = x as i64;
        let mut n = x as i64;
        let mut invertido: i64 = 0;
        while n>0{
            let digito = n % 10;
            invertido = invertido * 10 + digito;
            n /= 10;
        }
        original == invertido
    }
}