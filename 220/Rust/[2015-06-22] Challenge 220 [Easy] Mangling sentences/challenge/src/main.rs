extern crate regex;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input1 = "This challenge doesn't seem so hard.".to_string();
    let input2 = "There are more things between heaven and earth, Horatio, than are dreamt of in your philosophy.".to_string();
    let output1 = "Hist aceeghlln denos't eems os adhr.".to_string();
    let output2 = "Eehrt aer emor ghinst beeentw aeehnv adn aehrt, Ahioort, ahnt aer ademrt fo in oruy hhilooppsy.".to_string();

    if challenge(input1) == output1 {
    	println!("WIN challenge1");
    }
    if challenge(input2) == output2 {
    	println!("WIN challenge2");
    }

}

fn challenge(input: String) -> String{
    let words = input.split(' ');
    let mut ret  = input.clone();
    for word in words {
        let mut non_alpha_numeric = HashMap::new();
        let mut uppercase = HashMap::new();
        for i in 0..word.len() {
            if word.chars().nth(i).unwrap().is_uppercase() {
                uppercase.insert(i, true);
            }
            if !word.chars().nth(i).unwrap().is_alphanumeric() {
                non_alpha_numeric.insert(i, word.chars().nth(i).unwrap());
            }
        }
        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        let clean_word = re.replace_all(word, "");
        let mut sortedclean_word: Vec<char> = clean_word.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
        sortedclean_word.sort();
        for (key, value) in non_alpha_numeric {
            sortedclean_word.insert(key, value);
        }
        for (key, value) in uppercase {
            sortedclean_word[key] = sortedclean_word[key].to_uppercase().next().unwrap();
        }
        let s: String = sortedclean_word.into_iter().collect();
        ret = ret.replace(word, &s);
    }
    ret
}
