use std::collections::HashMap;
use std::fs::File;

fn gen_morse() -> HashMap<&'static str, char>{
    let mut morse_table: HashMap<&str, char> = HashMap::new();
    morse_table.insert("c.-", 'A');
    morse_table.insert("c-...", 'B');
    morse_table.insert("c-.-.", 'C');
    morse_table.insert("c-..", 'D');
    morse_table.insert("c.", 'E');
    morse_table.insert("c..-.", 'F');
    morse_table.insert("c--.", 'G');
    morse_table.insert("c....", 'H');
    morse_table.insert("c..", 'I');
    morse_table.insert("c.---", 'J');
    morse_table.insert("c-.-", 'K');
    morse_table.insert("c.-..", 'L');
    morse_table.insert("c--", 'M');
    morse_table.insert("c-.", 'N');
    morse_table.insert("c---", 'O');
    morse_table.insert("c.--.", 'P');
    morse_table.insert("c--.-", 'Q');
    morse_table.insert("c.-.", 'R');
    morse_table.insert("c...", 'S');
    morse_table.insert("c-", 'T');
    morse_table.insert("c..-", 'U');
    morse_table.insert("c...-", 'V');
    morse_table.insert("c.--", 'W');
    morse_table.insert("c-..-", 'X');
    morse_table.insert("c....-", 'Y');
    morse_table.insert("c--..", 'Z');
    morse_table.insert("p.-", 'a');
    morse_table.insert("p-...", 'b');
    morse_table.insert("p-.-.", 'c');
    morse_table.insert("p-..", 'd');
    morse_table.insert("p.", 'e');
    morse_table.insert("p..-.", 'f');
    morse_table.insert("p--.", 'g');
    morse_table.insert("p....", 'h');
    morse_table.insert("p..", 'i');
    morse_table.insert("p.---", 'j');
    morse_table.insert("p-.-", 'k');
    morse_table.insert("p.-..", 'l');
    morse_table.insert("p--", 'm');
    morse_table.insert("p-.", 'n');
    morse_table.insert("p---", 'o');
    morse_table.insert("p.--.", 'p');
    morse_table.insert("p--.-", 'q');
    morse_table.insert(".-.", 'r');
    morse_table.insert("p...", 's');
    morse_table.insert("p-", 't');
    morse_table.insert("p..-", 'u');
    morse_table.insert("p...-", 'v');
    morse_table.insert("p.--", 'w');
    morse_table.insert("p-..-", 'x');
    morse_table.insert("p....-", 'y');
    morse_table.insert("p--..", 'z');
    morse_table.insert("p-----", '0');
    morse_table.insert("p.----", '1');
    morse_table.insert("p..---", '2');
    morse_table.insert("p...--", '3');
    morse_table.insert("p....-", '4');
    morse_table.insert("p.....", '5');
    morse_table.insert("p-....", '6');
    morse_table.insert("p--...", '7');
    morse_table.insert("p---..", '8');
    morse_table.insert("p----.", '9');
    morse_table.insert("p.-.-.-", '.');
    morse_table.insert("p--..--", ',');
    morse_table.insert("p..--..", '?');
    morse_table.insert("p.---.", '\'');
    morse_table.insert("p-.-.--", '!');
    morse_table.insert("p-..-.", '/');
    morse_table.insert("b-..-.", '\\');
    morse_table.insert("p-.--.", '(');
    morse_table.insert("p-.--.-", ')');
    morse_table.insert("p.-...", '&');
    morse_table.insert("p---...", ':');
    morse_table.insert("p-.-.-.", ';');
    morse_table.insert("p-...-", '=');
    morse_table.insert("p.-.-.", '+');
    morse_table.insert("p-....-", '-');
    morse_table.insert("p..--.-", '_');
    morse_table.insert("p.-..-.", '"');
    morse_table.insert("p...-.-", '$');
    morse_table.insert("p.--.-.", '@');
    morse_table // Return the Hashmap
}

fn main() {
    let morse_table = gen_morse();
    let mut hellified: HashMap<char, &str> = HashMap::new();
    for (key, value) in morse_table {
        hellified.insert(value, key);
    }
}
