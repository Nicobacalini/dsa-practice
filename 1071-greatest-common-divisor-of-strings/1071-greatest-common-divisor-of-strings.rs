impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return String::new();
        }
        let gcd_length = Self::gcd(str1.len(), str2.len());

        str1[..gcd_length].to_string()
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
