impl Solution {
     pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string(); 
        }

        let primera = &strs[0];
        let primera_bytes = primera.as_bytes();


        for i in 0..primera_bytes.len() {
            let letra_objetivo = primera_bytes[i];

            for palabra in strs.iter().skip(1) {
                let palabra_bytes = palabra.as_bytes();

                if i >= palabra_bytes.len() || palabra_bytes[i] != letra_objetivo {
                    return primera[..i].to_string();
                }
            }
        }
        primera.to_string()
    }
}