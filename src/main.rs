use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut morse_table: HashMap<&str, char> = HashMap::new();
    morse_table.insert(".-", 'A');
    morse_table.insert("-...", 'B');
    morse_table.insert("-.-.", 'C');
    morse_table.insert("-..", 'D');
}
