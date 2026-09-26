impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn valor(c: u8) -> i32 {
            match c {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        }

    let bytes = s.as_bytes();
    let mut total = 0;
    
    for i in 0..bytes.len(){
        let actual = valor(bytes[i]);
        if i + 1 < bytes.len() && actual < valor(bytes[i+1]){
            total -= actual;
        }
        else{
            total += actual;
        }
    }
    total
    }
}