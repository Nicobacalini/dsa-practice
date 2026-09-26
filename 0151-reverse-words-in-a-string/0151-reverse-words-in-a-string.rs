impl Solution {
    pub fn reverse_words(s: String) -> String {
        let bytes = s.as_bytes(); // Convertimos a bytes para recorrer por índices
        let mut i = bytes.len() as i32 - 1;
        let mut resultado = String::new();

        while i >= 0 {
            while i >= 0 && bytes[i as usize] == b' ' {
                i -= 1;
            }
            if i < 0 {
                break;
            }
            let fin = i as usize;

            while i >= 0 && bytes[i as usize] != b' ' {
                i -= 1;
            }
            let inicio = (i + 1) as usize;
            let palabra = &s[inicio..=fin];

            if !resultado.is_empty() {
                resultado.push(' ');
            }
            resultado.push_str(palabra);
        }

        resultado
    }
}